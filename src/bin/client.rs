use std::io::prelude::*;
use std::net::TcpStream;

fn main() {
    let mut stream = TcpStream::connect("localhost:4444").unwrap();
    
    stream.write(&[1]).unwrap();
}