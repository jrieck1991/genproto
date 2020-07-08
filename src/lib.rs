use bincode;
use serde::{Deserialize, Serialize};
use byteorder::{BigEndian, ByteOrder};


#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct PutData {
    pub key: Vec<u8>,
    pub value: Vec<u8>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct GetData {
    pub key: Vec<u8>,
}

// form get request and serialize into bytes
pub fn form_get(key: &str) -> Vec<u8> {

    // form struct
    let data = GetData{
        key: key.as_bytes().to_vec(),
    };

    serialize(data)
}

// form put request and serialize into bytes
pub fn form_put(key: &str, value: &str) -> Vec<u8> {

    // form struct
    let data = PutData{
        key: key.as_bytes().to_vec(),
        value: value.as_bytes().to_vec(),
    };

    serialize(data)
}

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