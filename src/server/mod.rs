use bincode;
use byteorder::{BigEndian, ByteOrder};
use serde::{Deserialize, Serialize};
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
mod store;

pub struct Server {
    addr: String,
    store: store::Storage,
}

impl Server {
    pub fn new(listen_addr: &str) -> Server {
        Server {
            addr: listen_addr.to_string(),
            store: store::Storage::new(),
        }
    }

    pub fn start(self: &mut Self) {
        let listener = TcpListener::bind(&self.addr).unwrap();

        for stream in listener.incoming() {
            let stream = match stream {
                Ok(stream) => stream,
                Err(e) => {
                    println!("connection error: {}", e);
                    continue;
                }
            };
            self.handle(stream);
        }
    }

    fn handle(self: &mut Self, mut stream: TcpStream) {
        loop {
            // read tag
            let mut tag_buf = [0; 1];
            match stream.read_exact(&mut tag_buf) {
                Ok(()) => (),
                Err(e) => {
                    println!("error: {}", e);
                    return;
                }
            }

            // verify tag
            if tag_buf[0] != TAG[0] && tag_buf.len() != 1 {
                println!("unknown tag");
                return;
            }

            // read len
            let mut len_buf = [0; 4];
            match stream.read_exact(&mut len_buf) {
                Ok(()) => (),
                Err(e) => {
                    println!("error: {}", e);
                    return;
                }
            }

            // convert to u32
            let len = BigEndian::read_u32(&len_buf);
            let len_usize = len as usize;

            // read data
            let mut data_buf = vec![0; len_usize];
            match stream.read_exact(&mut data_buf) {
                Ok(()) => (),
                Err(e) => {
                    println!("error: {}", e);
                    return;
                }
            }

            // deserialize bytes into struct
            let data = parse_request(data_buf);

            // action on request
            self.action(data);
        }
    }

    // action on data received
    fn action(self: &mut Self, data: Data) {
        match data.value {
            Some(value) => {
                println!("put received");

                self.store.set(
                    String::from_utf8(data.key).unwrap(),
                    String::from_utf8(value).unwrap(),
                );
            }
            None => {
                println!("get received");

                let value = match self.store.get(String::from_utf8(data.key).unwrap()) {
                    Some(value) => value,
                    None => {
                        println!("no match found");
                        return;
                    }
                };

                println!("match found, value: {}", value);
            }
        }
    }
}

// protocol tag
const TAG: &[u8; 1] = b"\x00";

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Data {
    pub key: Vec<u8>,
    pub value: Option<Vec<u8>>,
}

// parse serialized request and return struct
pub fn parse_request(data: Vec<u8>) -> Data {
    let de_data: Data = bincode::deserialize(&data).unwrap();

    de_data
}

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
