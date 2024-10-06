use clap::Parser;
use std::net::{SocketAddr, UdpSocket};

/// A server for p2p discovery
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Port to start the server on
    #[arg(short, long, default_value = "7777")]
    port: u16,
}

enum MagOperation {
    Regester = 0x01,
    Deregester = 0x02,
    Fetch = 0x03,
}

struct MagRequest {
    magic: [u8; 4],    // Magic number
    version: u32,      // Version
    catagory: [u8; 4], //
    operation: u32,
}

impl MagRequest {
    fn new() -> Self {}
}

struct Peer {
    catagory: [u8; 4],
    addr: SocketAddr,
}

fn main() {
    let args: Args = Args::parse();

    let server_ip: String = String::from(format!("[::1]:{}", args.port));
    let server_socket: UdpSocket = UdpSocket::bind(server_ip).unwrap();
    let mut peer_list: Vec<Peer> = Vec::new();

    let mut buf: Vec<u8> = vec![0; 1024];
    loop {
        let (bytes, addr) = match server_socket.recv_from(&mut buf) {
            Ok(x) => x,
            Err(e) => {
                println!("Error: {e}");
                continue;
            }
        };
    }
}
