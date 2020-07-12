use bincode;
use byteorder::{BigEndian, ByteOrder};
use serde::{Deserialize, Serialize};
use std::io::prelude::*;
use std::net::TcpStream;

// protocol tag
const TAG: &[u8; 1] = b"\x00";

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Data {
    pub key: Vec<u8>,
    pub value: Option<Vec<u8>>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Request {
    pub action: String,
    pub data: Data,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Response {
    pub data: Data,
}

// parse serialized request and return
fn parse_request(data: Vec<u8>) -> Option<Request> {
    let request: Request = match bincode::deserialize(&data) {
        Ok(request) => request,
        Err(e) => {
            println!("parse_request error: {}", e);
            return None;
        }
    };

    Some(request)
}

// parse serialized response and return
fn parse_response(data: Vec<u8>) -> Option<Response> {
    let res: Response = match bincode::deserialize(&data) {
        Ok(res) => res,
        Err(e) => {
            println!("parse_request error: {}", e);
            return None;
        }
    };

    Some(res)
}

// serialize to bytes, tlv
fn serialize<T: Serialize>(data: T) -> Vec<u8> {
    // serialize struct
    let data_bytes = bincode::serialize(&data).unwrap();

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

    payload
}

pub fn read_request(stream: &mut TcpStream) -> Option<Request> {

    // read bytes into a buffer from the tcp stream
    let buf = match read_stream(stream) {
        Some(buf) => buf,
        None => return None,
    };

    // deserialize bytes into request
    match parse_request(buf) {
        Some(request) => Some(request),
        None => None,
    }
}

pub fn read_response(stream: &mut TcpStream) -> Option<Response> {

    // read bytes into a buffer from the tcp stream
    let buf = match read_stream(stream) {
        Some(buf) => buf,
        None => return None,
    };

    // deserialize bytes into request
    match parse_response(buf) {
        Some(res) => Some(res),
        None => None,
    }
}

// read from the tcp stream and parse a request
fn read_stream(stream: &mut TcpStream) -> Option<Vec<u8>> {
    // read tag
    let mut tag_buf = [0; 1];
    match stream.read_exact(&mut tag_buf) {
        Ok(()) => (),
        Err(e) => {
            println!("error: {}", e);
            return None;
        }
    }

    // verify tag
    if tag_buf[0] != TAG[0] && tag_buf.len() != 1 {
        println!("unknown tag");
        return None;
    }

    // read len
    let mut len_buf = [0; 4];
    match stream.read_exact(&mut len_buf) {
        Ok(()) => (),
        Err(e) => {
            println!("error: {}", e);
            return None;
        }
    }

    // convert to u32
    let len = BigEndian::read_u32(&len_buf);
    let len_usize = len as usize;

    // read data
    let mut data_buf = vec![0; len_usize];
    match stream.read_exact(&mut data_buf) {
        Ok(data_buf) => data_buf,
        Err(e) => {
            println!("error: {}", e);
            return None;
        }
    };

    Some(data_buf)
}

// write to a tcp stream with a request
pub fn write_request(stream: &mut TcpStream, req: Request) {
    // serialize request into bytes
    let payload = serialize(req);

    // write to tcp stream
    match stream.write(&payload) {
        Ok(n) => {
            println!("{} bytes written", n);
        }
        Err(e) => {
            println!("write_stream error: {}", e);
        }
    }
}

// write to a tcp stream with a request
pub fn write_response(stream: &mut TcpStream, res: Response) {
    // serialize request into bytes
    let payload = serialize(res);

    // write to tcp stream
    match stream.write(&payload) {
        Ok(n) => {
            println!("{} bytes written", n);
        }
        Err(e) => {
            println!("write_stream error: {}", e);
        }
    }
}
