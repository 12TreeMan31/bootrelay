use crate::request::{Kind, Listing, Request};
use clap::Parser;
use std::net::{Ipv6Addr, SocketAddr, UdpSocket};

mod request;

/// A server for p2p discovery
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Port to start the server on
    #[arg(short, long, default_value = "7777")]
    port: u16,
}

fn debug() {
    let cat: [u8; 4] = [b'M', b'A', b'I', b'C'];
    let ss = Request {
        req: Kind::Register,
        catagory: cat,
        addr: SocketAddr::new(
            Ipv6Addr::new(0000, 0000, 0000, 0000, 0000, 0000, 0000, 0000).into(),
            7777,
        ),
    };
    let json: String = serde_json::to_string(&ss).unwrap();
    println!("{}", json);
}

fn main() {
    debug();
    let args: Args = Args::parse();
    let server_ip: String = String::from(format!("[::]:{}", args.port));
    println!("Starting server on {}", server_ip);
    let server_socket: UdpSocket = UdpSocket::bind(server_ip).unwrap();

    //let mut known_clients: HashMap<SocketAddr, ()> = HashMap::new();
    let mut known_clients: Listing = Listing::new();

    let mut buf: Vec<u8> = vec![0; 1024];
    loop {
        let (bytes, addr) = match server_socket.recv_from(&mut buf) {
            Ok(x) => x,
            Err(e) => {
                println!("Error: {e}");
                continue;
            }
        };
        let mut req: Request = match Request::from_slice(&buf[..bytes]) {
            Some(x) => x,
            None => continue,
        };

        req.addr = addr;

        match req.req {
            Kind::Deregister => known_clients.deregister(&req),
            Kind::Register => known_clients.register(&req),
            Kind::Fetch => {
                let res: Box<String> = known_clients.fetch(req.catagory);
                server_socket.send_to(res.as_bytes(), addr).unwrap();
            }
        }
    }
}
