mod client;

fn main() {
    let c = client::Client::new("localhost:4444");

    c.put("test-key", "test-value");

    let value = c.get("test-key");

    assert_eq!(value, "test-value");
}
