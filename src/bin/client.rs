use byteorder::{BigEndian, ByteOrder};
use std::io::prelude::*;
use std::net::TcpStream;

const TAG: &[u8; 1] = b"\x00";

fn main() {
    // connect to server
    let mut stream = TcpStream::connect("localhost:4444").unwrap();

    // generate some data
    let data = String::from("hello world");
    let data_bytes = data.as_bytes();

    // get len of data
    let len = data_bytes.len();
    let len_u32 = len as u32;
    let mut len_buf = [0; 4];

    // serialize len
    BigEndian::write_u32(&mut len_buf, len_u32);

    // form payload
    let mut payload: Vec<u8> = Vec::new();
    payload.extend_from_slice(TAG);
    payload.extend_from_slice(&len_buf);
    payload.extend_from_slice(&data_bytes);

    stream.write(&payload).unwrap();
}
