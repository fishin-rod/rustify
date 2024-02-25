use std::collections::HashMap;

use serde::{Serialize, Deserialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Cache<T> {
    pub cache: HashMap<String, T>,
}

impl<T> Cache<T> where 
    T: Clone,
{
    pub fn new() -> Cache<T> {
        Cache {
            cache: HashMap::with_capacity(12),
        }
    }

    pub fn get(&self, key: &str) -> Option<T> {
        // Just for safety
        if !self.cache.contains_key(key) {
            return None;
        }
        let value = self.cache.get(key).expect("There was an issue getting the key? THIS IS BAD!").clone();
        Some(value)
    }

    pub fn add(&mut self, key: &str, val: T) {
        if self.cache.len() >= 10 {
            let last_key = self.cache.keys().last().unwrap().to_string();
            self.cache.remove(&last_key);

        }
        self.cache.insert(key.to_string(), val);
    }

    pub fn contains_key(&self, key: &str) -> bool {
        self.cache.contains_key(key)
    }
}