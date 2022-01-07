#[warn(unused_variables)]
#[warn(unused_imports)]
use std::{net::{IpAddr, SocketAddr, Ipv4Addr}};
use std::io;
use tokio::net::UdpSocket;
use tokio::time::{self, Duration};
use std::time::Instant;
use std::io::ErrorKind;
use tokio::time::Timeout;
#[tokio::main]
async fn main(){
    let local_ip = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 1900);
    perform_m_search(local_ip).await;
}
#[warn(unused_variables)]
#[warn(unused_imports)]
async fn perform_m_search(local_ip: SocketAddr) -> io::Result<()>{
    let ip_str = local_ip.to_string();
    let socket = UdpSocket::bind(ip_str).await?;
    let address = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(239, 255, 255, 250)), 1900);
    let ssdp_request = "M-SEARCH * HTTP/1.1\r\nHOST:239.255.255.250:1900\r\nMAN:\"ssdp:discover\"\r\nST:upnp:rootdevice\r\nMX:5\r\n\r\n";
    //socket.connect(address).await?;
    let _resp = socket.send_to(ssdp_request.as_bytes(), address).await?;
    let timeout = Duration::from_secs(10).as_secs();
    let start = Instant::now();
    let mut delay = time::sleep(Duration::from_millis(50));
    loop {
        tokio::select! {
            _ = &mut delay => {
                println!("operation timed out");
                break;
            }
            _ = () => {
                    let mut buf = [0u8; 8196];
                    match socket.recv(&mut buf).await {
                        Ok(read) => {
                            println!("{}", std::str::from_utf8(&buf[..read]).unwrap());
                            continue;
                        }
                        Err(ref e) if e.kind() == ErrorKind::WouldBlock => {
                            continue;
                        }
                        Err(e) => {
                            return Err(e);
                        }
                    }
            }
        }
    }
    //let resp = socket.send_to(ssdp_request.as_bytes(), addr).await?;
}

/*
fn main() {
    let local_ip = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 1900);
    perform_m_search(local_ip);
}
*/

/*
fn perform_m_search(local_ip: SocketAddr){
    let socket = UdpSocket::bind(local_ip.to_string()).expect("Couldn't bind the address");
    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(239, 255, 255, 250)), 1900);
    let ssdp_request = "M-SEARCH * HTTP/1.1\r\nHOST:239.255.255.250:1900\r\nMAN:\"ssdp:discover\"\r\nST:upnp:rootdevice\r\nMX:5\r\n\r\n";
    socket.send_to(ssdp_request.as_bytes(), addr).expect("Couldn't send SSDP Request");
    let timeout = Duration::from_secs(10).as_secs();
    let start = Instant::now();

    loop{
        let now = Instant::now();
        if now.duration_since(start).as_secs() > timeout{
            break;
        }
        let mut buf = [0u8; 8192];
        match socket.recv(&mut buf){
            Ok(read) if read >= 8192 => {
                println!("Buffer exceeded!");
                break;
            }
            Ok(read) => {
                println!("{}", std::str::from_utf8(&buf[..read]).unwrap());
                continue;
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
*/
