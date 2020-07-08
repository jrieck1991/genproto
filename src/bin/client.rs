use genproto::{form_get, form_put};
use std::io::prelude::*;
use std::net::TcpStream;

fn main() {
    get("hello_world");
}

fn get(key: &str) -> Option<String> {
    // form get
    let get_data = form_get(key);

    // connect to server
    let mut stream = TcpStream::connect("localhost:4444").unwrap();

    // write get request
    stream.write(&get_data).unwrap();

    None
}

fn put(key: &str, value: &str) {
    // form put
    let put_data = form_put(key, value);

    // connect to server
    let mut stream = TcpStream::connect("localhost:4444").unwrap();

    // write payload
    stream.write(&put_data).unwrap();
}
