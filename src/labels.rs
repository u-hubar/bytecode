use std::{collections::HashMap, iter::FromIterator};

use crate::vm::Pointer;

#[derive(Debug)]
pub struct Labels(HashMap<String, Pointer>);

impl Labels {
    pub fn new() -> Labels {
        Labels(HashMap::new())
    }

    pub fn insert(&mut self, label_key: String, ip: Pointer) {
        match self.0.insert(label_key, ip) {
            Some(_) => panic!("Duplicated label!"),
            None => {},
        };
    }

    pub fn get(&self, label_key: &str) -> &Pointer {
        self.0.get(label_key).expect("Label doesn't exist.")
    }
}

impl FromIterator<(String, Pointer)> for Labels {
    fn from_iter<I: IntoIterator<Item = (String, Pointer)>>(iter: I) -> Self {
        let mut labels = HashMap::new();
        for (k, v) in iter {
            labels.insert(k, v);
        }
        Self(labels)
    }
}
