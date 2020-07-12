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
            // read request from stream
            let request = match lib::read_stream(&mut stream) {
                Some(request) => request,
                None => return,
            };

            // action on request
            self.action(request);
        }
    }

    // action on data received
    fn action(self: &mut Self, req: lib::Request) {
        let action = req.action.as_str();

        match action {
            "put" => {
                println!("put received");

                // get value from data struct
                let value = match req.data.value {
                    Some(value) => value,
                    None => {
                        println!("put requires value field set");
                        return;
                    }
                };

                // put data in storage
                self.store.set(
                    String::from_utf8(req.data.key).unwrap(),
                    String::from_utf8(value).unwrap(),
                );
            }
            "get" => {
                println!("get received");

                // get key and return value if found
                let value = match self.store.get(String::from_utf8(req.data.key).unwrap()) {
                    Some(value) => value,
                    None => {
                        println!("no match found");
                        return;
                    }
                };

                println!("match found, value: {}", value);
            }
            _ => {
                println!("{} didn't match any actions", action);
            }
        }
    }
}
