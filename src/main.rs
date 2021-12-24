use std::{net::{UdpSocket, IpAddr, SocketAddr, Ipv4Addr}, time::{Duration, Instant}};
use std::io::ErrorKind;

fn main() {
    let local_ip = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 1900);
    perform_m_search(local_ip);
}

fn perform_m_search(local_ip: SocketAddr){
    let socket = UdpSocket::bind(local_ip.to_string()).expect("Couldn't bind the address");
    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(239, 255, 255, 250)), 1900);
    let ssdp_request = "M-SEARCH * HTTP/1.1\r\nHOST:239.255.255.250:1900\r\nMAN:\"ssdp:discover\"\r\nST:upnp:rootdevice\r\nMX:5\r\n\r\n";
    socket.send_to(ssdp_request.as_bytes(), addr).expect("Couldn't send SSDP Request");
    let timeout = Duration::from_secs(10);
    let start = Instant::now();

    loop{
        let now = Instant::now();
        if (now - start) >= timeout{
            break;
        }
        let mut buf = [0u8; 8192];
        match socket.recv(&mut buf){
            Ok(read) if read == 8192 => {
                println!("Buffer exceeded!");
            }
            Ok(read) => {
                println!("{}", std::str::from_utf8(&buf[..read]).unwrap());
            }
            Err(e) if e.kind() == ErrorKind::WouldBlock || e.kind() == ErrorKind::TimedOut => {
                continue;
            }
            Err(e) => {
                println!("{}", e);
                break;
            }
        }
    }

}