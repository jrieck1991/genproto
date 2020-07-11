use std::collections::HashMap;

#[derive(Debug)]
pub struct Storage {
    map: HashMap<String, String>,
}

// Storage is just a wrapper for a map
// TODO: do something cooler
impl Storage {
    // new returns a chest: HashMap<String, String>
    pub fn new() -> Storage {
        Storage {
            map: HashMap::new(),
        }
    }

    // get returns value corresponding to key
    pub fn get(self: &Self, key: String) -> Option<String> {
        match self.map.get(&key) {
            None => None,
            Some(value) => Some(String::from(value)),
        }
    }

    // set will insert key:value pair into map, returns previous value for key
    pub fn set(self: &mut Self, key: String, value: String) -> Option<String> {
        match self.map.insert(key, value) {
            None => None,
            Some(prev_value) => Some(String::from(prev_value)),
        }
    }

    // delete will remove a key from the map, returning the deleted value
    pub fn delete(self: &mut Self, key: String) -> Option<String> {
        match self.map.remove(&key) {
            None => None,
            Some(deleted_value) => Some(String::from(deleted_value)),
        }
    }
}