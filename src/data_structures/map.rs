use std::collections::HashMap;
use std::hash::Hash;

pub struct Map<K, V> {
    elements: HashMap<K, V>,
}

impl<K: Eq + Hash, V> Map<K, V> {
    pub fn new() -> Self {
        Map { elements: HashMap::new() }
    }

    pub fn insert(&mut self, key: K, value: V) {
        self.elements.insert(key, value);
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        self.elements.get(key)
    }

    pub fn show_all(&self) where K: std::fmt::Display, V: std::fmt::Display {
        for (key, val) in &self.elements {
            println!("[{}] => {}", key, val);
        }
    }
}