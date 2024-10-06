use crate::request::{Kind, Request};
use clap::Parser;
use std::collections::HashMap;
use std::net::{SocketAddr, UdpSocket};

mod request;

/// A server for p2p discovery
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Port to start the server on
    #[arg(short, long, default_value = "7777")]
    port: u16,
}

fn main() {
    let args: Args = Args::parse();
    let server_ip: String = String::from(format!("[::]:{}", args.port));
    println!("Starting server on {}", server_ip);
    let server_socket: UdpSocket = UdpSocket::bind(server_ip).unwrap();

    let mut known_clients: HashMap<SocketAddr, ()> = HashMap::new();

    let mut buf: Vec<u8> = vec![0; 1024];
    loop {
        let (bytes, addr) = match server_socket.recv_from(&mut buf) {
            Ok(x) => x,
            Err(e) => {
                println!("Error: {e}");
                continue;
            }
        };
        let req: Request = match Request::from_slice(&buf[..bytes]) {
            Some(x) => x,
            None => continue,
        };

        println!("New connection: {:?}", req);

        match req.req {
            Kind::Deregister => req.deregister(&mut known_clients, addr),
            Kind::Fetch => {
                let res: Box<String> = Request::fetch(&mut known_clients);
                println!("{}", res);
                server_socket.send_to(res.as_bytes(), addr).unwrap();
            }
            Kind::Register => req.regester(&mut known_clients, addr),
        }
    }
}
