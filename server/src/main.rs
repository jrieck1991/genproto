mod server;

fn main() {
    // init server
    let mut s = server::Server::new("localhost:4444");

    // start listening for requests
    s.start()
}
