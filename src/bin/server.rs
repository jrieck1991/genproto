use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

fn main() {
    
    let listener = TcpListener::bind("localhost:4444").unwrap();

    for stream in listener.incoming() {
        handle(stream.unwrap());
    }
}

fn handle(mut stream: TcpStream) {
    stream.read(&mut [0; 128]).unwrap();
}