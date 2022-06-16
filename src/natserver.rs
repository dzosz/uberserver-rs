use tokio::net::UdpSocket;
use std::net::SocketAddr;
use std::io;
use std::str;
use log::{error,info,debug};


pub struct NATServer {
}

impl NATServer {
    pub async fn start(&self, port : u32) -> io::Result<()> {
        let addr = format!("0.0.0.0:{}", port).parse::<SocketAddr>().unwrap();
		info!("Awaiting UDP messages on port {}", port);
        let sock = UdpSocket::bind(addr).await?;

        let mut buf = [0; 1024];
        let response = "PONG".as_bytes();
        loop {
            let (len, addr) = sock.recv_from(&mut buf).await?;
            debug!("{:?} bytes receied from {:?}", len, addr);
            match str::from_utf8(&buf) {
                Ok(msg) => {
                     // TODO write test for parsing message
                    let content = NATServer::trim_message(msg);
                    sock.send(&response).await?;

                    // TODO callback with msg
                    // callback(msg, addr);
                },
                Err(e) => {
                    error!("NATServer received broken msg from {}, err={}", addr,e);
                }
            }
        }
    }
    fn trim_message(data : &str) -> &str {
        return data.trim_end_matches('\n').trim_end_matches(' ');
    }
}
