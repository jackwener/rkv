use std::collections::HashMap;

pub struct KV {
    map: HashMap<String, String>
}

impl KV {
    pub fn new() -> KV {
        KV {
            map: HashMap::new()
        }
    }

    pub fn get(&mut self, key: String) -> Option<&String> {
        self.map.get(&key)
    }

    pub fn set(&mut self, key: String, value: String) -> Option<String> {
        self.map.insert(key, value)
    }

    pub fn remove(&mut self, key: String) -> Option<String> {
        self.map.remove(&key)
    }
}