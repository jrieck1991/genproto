mod client;

fn main() {
    let c = client::Client::new("localhost:4444");

    let put_code = c.put("test-key", "test-value");

    println!("put code: {:?}", put_code);

    let (value, get_code) = c.get("test-key");

    println!("value: {:?}, code: {:?}", value, get_code);

    let del_code = c.delete("test-key");

    println!("del_code: {:?}", del_code);

    let (_, get_nf_code) = c.get("test-key");

    println!("get_code: {:?}", get_nf_code);
}
