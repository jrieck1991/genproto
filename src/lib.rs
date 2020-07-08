use bincode;
use byteorder::{BigEndian, ByteOrder};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Data {
    pub key: Vec<u8>,
    pub value: Option<Vec<u8>>,
}

// form get request and serialize into bytes
pub fn form_get(key: &str) -> Vec<u8> {
    // form struct
    let data = Data {
        key: key.as_bytes().to_vec(),
        value: None,
    };

    serialize(data)
}

// form put request and serialize into bytes
pub fn form_put(key: &str, value: &str) -> Vec<u8> {
    // form struct
    let data = Data {
        key: key.as_bytes().to_vec(),
        value: Some(value.as_bytes().to_vec()),
    };

    serialize(data)
}

// parse serialized request and return struct
pub fn parse_request(data: Vec<u8>) -> Data {
    let de_data: Data = bincode::deserialize(&data).unwrap();

    de_data
}

// protocol tag
const TAG: &[u8; 1] = b"\x00";

fn serialize<T: Serialize>(data: T) -> Vec<u8> {
    // serialize struct
    let data_bytes = bincode::serialize(&data).unwrap();

    // get len of data
    let len = data_bytes.len();
    let len_u32 = len as u32;
    let mut len_buf = [0; 4];

    // serialize len
    BigEndian::write_u32(&mut len_buf, len_u32);

    // form payload
    let mut payload: Vec<u8> = Vec::new();
    payload.extend_from_slice(TAG);
    payload.extend_from_slice(&len_buf);
    payload.extend_from_slice(&data_bytes);

    payload
}
