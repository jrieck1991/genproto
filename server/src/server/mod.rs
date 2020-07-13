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

    // start listening for incoming connections
    pub fn start(self: &mut Self) {
        let listener = TcpListener::bind(&self.addr).unwrap();

        // listen for incoming tcp conns
        for stream in listener.incoming() {
            let stream = match stream {
                Ok(stream) => stream,
                Err(e) => {
                    println!("connection error: {}", e);
                    continue;
                }
            };

            // handle stream
            self.handle(stream);
        }
    }

    // handle tcp stream lifetime
    fn handle(self: &mut Self, mut stream: TcpStream) {
        loop {
            // read request from stream
            let request = match lib::read_request(&mut stream) {
                Some(request) => request,
                None => return,
            };

            // action on request
            let res = self.action(request);

            // write response back to client
            lib::write_response(&mut stream, res);
        }
    }

    // action on request
    fn action(self: &mut Self, req: lib::Request) -> lib::Response {
        // init response
        let mut response = lib::Response {
            data: lib::Data {
                key: req.data.key.clone(),
                value: None,
            },
            code: lib::ResponseCode::Ok,
        };

        // convert action to str for matching
        let action = req.action.as_str();

        match action {
            "put" => {
                println!("put received");

                // get value from data struct
                match req.data.value {
                    Some(value) => {
                        // put data in storage
                        self.store.put(
                            &String::from_utf8(req.data.key).unwrap(),
                            &String::from_utf8(value).unwrap(),
                        );
                    }
                    None => {
                        response.code = lib::ResponseCode::BadRequest;
                        println!("put requires value field set");
                    }
                };
            }
            "get" => {
                println!("get received");

                // convert key from bytes to string
                let key = match String::from_utf8(req.data.key) {
                    Ok(key) => key,
                    Err(e) => {
                        response.code = lib::ResponseCode::BadRequest;
                        println!("conversion error: {}", e);
                        return response
                    }
                };

                // get key and return value if found
                match self.store.get(&key) {
                    Some(value) => {
                        response.data.value = Some(value.as_bytes().to_vec());
                        println!("match");
                    }
                    None => {
                        response.code = lib::ResponseCode::NotFound;
                        println!("no match");
                    }
                };
            }
            "delete" => {
                println!("delete received");

                // convert key from bytes to string
                let key = match String::from_utf8(req.data.key) {
                    Ok(key) => key,
                    Err(e) => {
                        response.code = lib::ResponseCode::BadRequest;
                        println!("conversion error: {}", e);
                        return response
                    }
                };

                // delete key from store
                match self.store.delete(&key) {
                    Some(deleted_value) => (),
                    None => {
                        response.code = lib::ResponseCode::NotFound;
                        println!("delete not found for key: {}", key)
                    },
                };
            }
            _ => {
                println!("{} didn't match any actions", action);
            }
        }

        response
    }
}
