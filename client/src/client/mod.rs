use std::net::{TcpListener, TcpStream};

pub struct Client {
    addr: String,
}

impl Client {
    pub fn new(server_addr: &str) -> Client {
        Client{
            addr: server_addr.to_string(), 
        }
    }

    pub fn get(&self, key: &str) -> String {

        let mut stream = TcpStream::connect(&self.addr).unwrap();

        String::from("hi")
    } 

    pub fn put(&self, key: &str, value: &str) {}
}