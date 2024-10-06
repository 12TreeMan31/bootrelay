use serde::{Deserialize, Serialize};
use serde_json;

use std::collections::HashMap;
use std::net::SocketAddr;

#[derive(Serialize, Deserialize, Debug)]
pub enum Kind {
    Register,
    Deregister,
    Fetch,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Request {
    pub req: Kind,
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
