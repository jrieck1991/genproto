fn main() {
    // init server
    let mut s = genproto::server::Server::new("localhost:4444");

    // start listening for requests
    s.start()
}
