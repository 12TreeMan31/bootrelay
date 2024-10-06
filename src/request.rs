use serde::{Deserialize, Serialize};
use serde_json;

use std::collections::HashMap;
use std::net::{IpAddr, Ipv6Addr, SocketAddr};

use std::time::Instant;

#[derive(Serialize, Deserialize, Debug)]
pub enum Kind {
    Register,
    Deregister,
    Fetch,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Request {
    pub req: Kind,
    catagory: [u8; 4],
    addr: SocketAddr,
}

impl Request {
    pub fn from_slice(buf: &[u8]) -> Option<Self> {
        let res: Option<Request> = serde_json::from_slice(buf).unwrap_or_else(|e| {
            println!("{e}");
            return None;
        });
        return res;
    }

    pub fn regester(&self, listing: &mut HashMap<SocketAddr, ()>, addr: SocketAddr) {
        listing.insert(addr, ());
    }
    pub fn deregister(&self, listing: &mut HashMap<SocketAddr, ()>, addr: SocketAddr) {
        listing.remove(&addr);
    }
    pub fn fetch(listing: &mut HashMap<SocketAddr, ()>) -> Box<String> {
        let mut st: String = String::new();
        for (i, _) in listing.iter() {
            st.push_str(&i.to_string());
        }
        return Box::new(st);
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct PeerInfo {
    time_alive: f64,
    ip: [u8; 16],
    port: u16,
}

impl PeerInfo {
    pub fn new(addr: &SocketAddr, time_alive: Instant) -> Option<Self> {
        // Converts SocketAddr into an ipv6 addr
        // We don't need to handle ipv4 as Linux converts it to a ipv6 format
        let v6: Ipv6Addr = match addr.ip() {
            IpAddr::V6(a) => a,
            IpAddr::V4(_) => return None,
        };

        let peer: PeerInfo = PeerInfo {
            time_alive: time_alive.elapsed().as_secs_f64(),
            ip: v6.octets(),
            port: addr.port(),
        };

        return Some(peer);
    }
}

pub struct Listing {
    list: HashMap<[u8; 4], HashMap<SocketAddr, Instant>>,
}

impl Listing {
    pub fn new() -> Self {
        Listing {
            list: HashMap::new(),
        }
    }

    pub fn register(&mut self, req: &Request) {
        match self.list.get_mut(&req.catagory) {
            Some(x) => {
                x.insert(req.addr, Instant::now());
            }
            // Create catagory if it does not exist
            None => {
                self.list.insert(req.catagory, HashMap::new());
                self.register(&req);
            }
        };
    }

    pub fn deregister(&mut self, req: &Request) {
        match self.list.get_mut(&req.catagory) {
            Some(x) => {
                x.remove(&req.addr);
            }
            None => (),
        }
    }

    /*pub fn fetch(&self) -> Box<String> {
        let mut json_data: String = String::new();
    }*/
}
