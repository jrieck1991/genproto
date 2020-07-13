use tokio::net::{TcpListener, TcpStream};
use tokio::prelude::*;
use tokio::stream::StreamExt;
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
    pub async fn start(self: &mut Self) {

        // bind to addr
        let mut listener = match TcpListener::bind(&self.addr).await {
            Ok(listener) => listener,
            Err(e) => {
                println!("bind err: {}", e);
                return
            }
        };

        let server = async move {

            // listen for incoming connections
            let mut incoming = listener.incoming();
            while let Some(stream_res) = incoming.next().await {
                // handle tcp streams
                match stream_res {
                    Ok(stream) => self.handle(stream.into()),
                    Err(e) => {
                        println!("accept err: {}", e);
                        return
                    }
                }
            }
        };

        // start server and block
        server.await;
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
                        return response;
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
                        return response;
                    }
                };

                // delete key from store
                match self.store.delete(&key) {
                    Some(_deleted_value) => (),
                    None => {
                        response.code = lib::ResponseCode::NotFound;
                        println!("delete not found for key: {}", key)
                    }
                };
            }
            _ => {
                println!("{} didn't match any actions", action);
            }
        }

        response
    }
}
