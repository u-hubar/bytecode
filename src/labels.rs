use std::{collections::HashMap, iter::FromIterator};

use crate::{vm::Pointer, errors::ParseError};

#[derive(Debug)]
pub struct Labels(HashMap<String, Pointer>);

impl Labels {
    pub fn new() -> Labels {
        Labels(HashMap::new())
    }

    pub fn insert(&mut self, label_name: String, ip: Pointer) -> Result<(), ParseError> {
        match self.0.insert(label_name, ip) {
            Some(_) => Err(ParseError::DuplicatedLabel),
            None => Ok(()),
        }
    }

    pub fn get(&self, label_name: &str) -> Result<&Pointer, ParseError> {
        self.0.get(label_name).ok_or(ParseError::LabelNotFound)
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
