mod client;

fn main() {
    let c = client::Client::new("localhost:4444");

    c.put("test-key", "test-value");

    let value = c.get("test-key");

    println!("value: {:#?}", value);
}
