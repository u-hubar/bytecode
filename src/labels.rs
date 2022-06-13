use std::{collections::HashMap, iter::FromIterator};

use crate::vm::Pointer;

#[derive(Debug)]
pub struct Labels(HashMap<String, Pointer>);

impl Labels {
    pub fn new() -> Labels {
        Labels(HashMap::new())
    }

    pub fn insert(&mut self, label_name: String, ip: Pointer) {
        match self.0.insert(label_name, ip) {
            Some(_) => panic!("Duplicated label!"),
            None => {},
        };
    }

    pub fn get(&self, label_name: &str) -> &Pointer {
        self.0.get(label_name).expect("Label doesn't exist.")
    }
}

impl FromIterator<(String, Pointer)> for Labels {
    fn from_iter<I: IntoIterator<Item = (String, Pointer)>>(iter: I) -> Self {
        let mut labels = HashMap::new();
        for (label_name, ip) in iter {
            labels.insert(label_name, ip);
        }
        Self(labels)
    }
}
