fn main() {

    // init server
    let s = genproto::server::Server::new("localhost:4444");

    // start listening for requests
    s.start()
}
