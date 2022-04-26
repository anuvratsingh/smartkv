use std::collections::HashMap;

#[derive(Default)]
pub struct SmartKV {
    map: HashMap<String, String>,
}

impl SmartKV {
    pub fn new() -> SmartKV {
        SmartKV {
            map: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: String, value: String) {
        self.map.insert(key, value);
    }

    pub fn get(&self, key: String) -> Option<String> {
        self.map.get(&key).cloned()
    }

    pub fn remove(&mut self, key: String) {
        let _ = self.map.remove(&key);
    }
}
