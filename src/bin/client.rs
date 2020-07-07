use byteorder::{BigEndian, ByteOrder};
use std::io::prelude::*;
use std::net::TcpStream;

fn main() {
    let mut stream = TcpStream::connect("localhost:4444").unwrap();

    // write tag
    stream.write(b"\x00").unwrap();

    // generate some data
    let data = String::from("hello world");
    let data_bytes = data.as_bytes();

    // get len of data
    let len = data_bytes.len();
    let len_u32 = len as u32;
    let mut len_buf = [0; 4];

    // serialize len
    BigEndian::write_u32(&mut len_buf, len_u32);
    stream.write(&len_buf).unwrap();

    // write data
    stream.write(&data_bytes).unwrap();
}
