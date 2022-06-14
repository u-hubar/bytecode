use std::{collections::HashMap, iter::FromIterator};

use crate::{vm::Pointer, errors::ParseError};

#[derive(Debug, PartialEq)]
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
        let mut labels = Labels::new();
        for (label_name, ip) in iter {
            labels.insert(label_name, ip).unwrap();
        }

        labels
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new() {
        let labels = Labels::new();

        assert!(labels.0.is_empty());
    }

    #[test]
    fn insert() {
        let mut labels = Labels::new();

        labels.insert("LOOP".to_string(), 5).unwrap();

        assert!(labels.0.contains_key("LOOP"));
    }

    #[test]
    fn insert_should_return_error_when_label_duplicated() {
        let mut labels = Labels::new();

        labels.insert("LOOP".to_string(), 5).unwrap();

        assert!(labels.insert("LOOP".to_string(), 5).is_err());
    }

    #[test]
    fn get() {
        let mut labels = Labels::new();

        labels.insert("LOOP".to_string(), 5).unwrap();

        assert_eq!(labels.get("LOOP").unwrap(), &5);
    }

    #[test]
    fn get_should_return_error_when_key_not_presented() {
        let labels = Labels::new();

        assert!(labels.get("LOOP").is_err());
    }

    #[test]
    fn from_iter() {
        let labels_vec: Vec<&str> = vec!["LOOP1", "LOOP2"];

        let labels = labels_vec
            .iter()
            .enumerate()
            .map(move |(i, y)| (y.to_string(), i))
            .collect::<Labels>();

        println!("{:?}", labels);

        assert_eq!(labels.get("LOOP1").unwrap(), &0);
        assert_eq!(labels.get("LOOP2").unwrap(), &1);
    }

    #[test]
    #[should_panic(expected = "Label duplicate found during parsing.")]
    fn from_iter_should_panic_while_parsing_duplicated_labels() {
        let labels_vec: Vec<&str> = vec!["LOOP1", "LOOP1"];

        let _ = labels_vec
            .iter()
            .enumerate()
            .map(move |(i, y)| (y.to_string(), i))
            .collect::<Labels>();
    }
}
