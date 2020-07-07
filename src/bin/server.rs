use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use byteorder::{ByteOrder, BigEndian};

fn main() {
    
    let listener = TcpListener::bind("localhost:4444").unwrap();

    for stream in listener.incoming() {
        handle(stream.unwrap());
    }
}

fn handle(mut stream: TcpStream) {

    // read tag
    let mut tag_buf = [0; 1];
    stream.read_exact(&mut tag_buf).unwrap();
    if tag_buf[0] != b"\x00"[0] && tag_buf.len() != 1 {
        println!("unknown tag");
        return
    }

    // read len
    let mut len_buf = [0; 4];
    stream.read_exact(&mut len_buf).unwrap();

    // convert to u32
    let len = BigEndian::read_u32(&len_buf);
    let len_usize = len as usize;

    // read data
    let mut data_buf = vec![0; len_usize];
    stream.read_exact(&mut data_buf).unwrap();

    println!("{:?}", String::from_utf8(data_buf).unwrap());
}