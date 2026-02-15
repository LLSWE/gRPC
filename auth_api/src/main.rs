use handler::handle_http;
use std::net::{SocketAddr, TcpListener};

mod config;
mod db;
mod handler;

fn main() {
    let addrs = [
        SocketAddr::from(([127, 0, 0, 1], 80)),
        SocketAddr::from(([127, 0, 0, 1], 443)),
    ];

    let listener = TcpListener::bind(&addrs[..]).unwrap;
    for streams in listener.incoming() {
        println!("connection received");
    }
}
