use byteorder::{BigEndian, ByteOrder};
use genproto::{parse_request, Data};
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

fn main() {
    let listener = TcpListener::bind("localhost:4444").unwrap();

    for stream in listener.incoming() {
        let stream = match stream {
            Ok(stream) => stream,
            Err(e) => {
                println!("connection error: {}", e);
                continue;
            }
        };
        handle(stream);
    }
}

// protocol tag
const TAG: &[u8; 1] = b"\x00";

fn handle(mut stream: TcpStream) {
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
        action(data);
    }
}

// action on data received
fn action(data: Data) {
    match data.value {
        Some(_val) => {
            // put
            println!("put received");
        }
        None => {
            // get
            println!("get received");
        }
    }
}
