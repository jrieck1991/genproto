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
    pub fn get(&self, key: &str) -> Option<String> {
        // form get request
        let req = lib::Request {
            action: String::from("get"),
            data: lib::Data {
                key: key.as_bytes().to_vec(),
                value: None,
            },
        };

        // connect to store
        let mut stream = TcpStream::connect(&self.addr).unwrap();

        // write request
        lib::write_request(&mut stream, req);

        // wait for response
        let res = match lib::read_response(&mut stream) {
            Some(res) => res,
            None => return None,
        };

        // check for value in response
        let value = match res.data.value {
            Some(value) => value,
            None => return None,
        };

        // convert to string
        let parsed_value = String::from_utf8(value).unwrap();

        Some(parsed_value)
    }

    // put pair into store
    pub fn put(&self, _key: &str, _value: &str) {}
}
