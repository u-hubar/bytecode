use std::collections::HashMap;

pub struct Frame<T>(HashMap<String, T>);

impl<T: Copy> Frame<T> {
    pub fn new() -> Frame<T> {
        Frame(HashMap::new())
    }

    pub fn insert(&mut self, key: String, value: T) {
        self.0.insert(key, value);
    }

    pub fn get(&self, key: String) -> T {
        *self.0.get(&key).expect("Variable doesn't exist!")
    }
}
