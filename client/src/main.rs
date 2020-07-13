mod client;

fn main() {
    let c = client::Client::new("localhost:4444");

    let put_code = c.put("test-key", "test-value");

    println!("put code: {:?}", put_code);

    let (value, get_code) = c.get("test-key");

    println!("value: {:?}, code: {:?}", value, get_code);
}
