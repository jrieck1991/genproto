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
    pub fn get(&self, key: &str) -> String {
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
        lib::write_stream(&mut stream, req);

        String::from("hi")
    }

    // put pair into store
    pub fn put(&self, key: &str, value: &str) {}
}
