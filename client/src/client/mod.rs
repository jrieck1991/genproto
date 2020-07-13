use std::net::TcpStream;

pub struct Client {
    addr: String,
}

impl Client {
    pub fn new(server_addr: &str) -> Client {
        Client {
            addr: server_addr.to_string(),
        }
    }

    // return value from store
    pub fn get(&self, key: &str) -> (Option<String>, lib::ResponseCode) {
        // form get request
        let req = lib::Request {
            action: String::from("get"),
            data: lib::Data {
                key: key.as_bytes().to_vec(),
                value: None,
            },
        };

        // connect to store
        let mut stream = match TcpStream::connect(&self.addr) {
            Ok(stream) => stream,
            Err(e) => {
                println!("connection error: {}", e);
                return (None, lib::ResponseCode::NoResponse);
            }
        };

        // write request
        lib::write_request(&mut stream, req);

        // wait for response
        let res = match lib::read_response(&mut stream) {
            Some(res) => res,
            None => {
                println!("no response");
                return (None, lib::ResponseCode::NoResponse);
            }
        };

        // check response code
        match res.code {
            lib::ResponseCode::Ok => (),
            _ => {
                println!("unexpected response code: {:?}", &res.code);
                return (None, res.code);
            }
        }

        // check for value in response
        let value = match res.data.value {
            Some(value) => value,
            None => {
                println!("no value in response");
                return (None, lib::ResponseCode::NotFound);
            }
        };

        // convert to string
        let parsed_value = match String::from_utf8(value) {
            Ok(parsed_value) => parsed_value,
            Err(e) => {
                println!("conversion error: {}", e);
                return (None, lib::ResponseCode::BadRequest);
            }
        };

        (Some(parsed_value), lib::ResponseCode::Ok)
    }

    // put pair into store
    pub fn put(&self, key: &str, value: &str) -> lib::ResponseCode {
        // form put request
        let req = lib::Request {
            action: String::from("put"),
            data: lib::Data {
                key: key.as_bytes().to_vec(),
                value: Some(value.as_bytes().to_vec()),
            },
        };

        // connect to store
        let mut stream = match TcpStream::connect(&self.addr) {
            Ok(stream) => stream,
            Err(e) => {
                println!("connection error: {}", e);
                return lib::ResponseCode::NoResponse;
            }
        };

        // write request
        lib::write_request(&mut stream, req);

        // wait for response
        let res = match lib::read_response(&mut stream) {
            Some(res) => res,
            None => {
                println!("no response");
                return lib::ResponseCode::NoResponse;
            }
        };

        return res.code;
    }

    // delete item matching key from store
    pub fn delete(&self, key: &str) -> lib::ResponseCode {
        // form delete request
        let req = lib::Request {
            action: String::from("delete"),
            data: lib::Data {
                key: key.as_bytes().to_vec(),
                value: None,
            },
        };

        // connect to store
        let mut stream = match TcpStream::connect(&self.addr) {
            Ok(stream) => stream,
            Err(e) => {
                println!("connection error: {}", e);
                return lib::ResponseCode::NoResponse;
            }
        };

        // write request
        lib::write_request(&mut stream, req);

        // wait for response
        let res = match lib::read_response(&mut stream) {
            Some(res) => res,
            None => {
                println!("no response");
                return lib::ResponseCode::NoResponse;
            }
        };

        return res.code;
    }
}
