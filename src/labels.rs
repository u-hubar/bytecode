use std::{collections::HashMap, iter::FromIterator};

use crate::vm::Pointer;

#[derive(Debug)]
pub struct Labels<'buf>(HashMap<&'buf str, Pointer>);

impl<'buf> Labels<'buf> {
    pub fn new() -> Labels<'buf> {
        Labels(HashMap::new())
    }

    pub fn insert(&mut self, label_key: &'buf str, ip: Pointer) {
        self.0.insert(label_key, ip);
    }

    pub fn get(&mut self, label_key: &str) -> &Pointer {
        self.0.get(label_key).expect("Label is not presented.")
    }
}

impl<'buf> FromIterator<(&'buf str, Pointer)> for Labels<'buf> {
    fn from_iter<I: IntoIterator<Item = (&'buf str, Pointer)>>(iter: I) -> Self {
        let mut labels = HashMap::new();
        for (k, v) in iter {
            labels.insert(k, v);
        }
        Self(labels)
    }
}
