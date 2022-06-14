use std::collections::{HashMap, VecDeque};

use crate::{vm::Pointer, errors::ParseError};

pub type VariableAddress = usize;

#[derive(Debug, PartialEq)]
pub struct Variables<'buf> {
    functions_locals: HashMap<&'buf str, HashMap<&'buf str, Pointer>>,
    queue: VecDeque<VariableAddress>,
}

impl<'buf> Variables<'buf> {
    pub fn new() -> Self {
        Self {
            functions_locals: HashMap::new(),
            queue: VecDeque::new(),
        }
    }

    pub fn insert_local(&mut self, func_name: &'buf str, var_name: &'buf str) {
        if !self.functions_locals.contains_key(func_name) {
            let locals_map = HashMap::new();
            self.functions_locals.insert(func_name, locals_map);
        }

        let func_map = self.functions_locals.get_mut(func_name).unwrap();

        if !func_map.contains_key(var_name) {
            func_map.insert(var_name, func_map.len());
        }

        self.queue.push_back(*func_map.get(var_name).unwrap());
    }

    pub fn queue_pop_front(&mut self) -> Result<VariableAddress, ParseError> {
        self.queue.pop_front().ok_or(ParseError::VariableNotFound)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new() {
        let variables = Variables::new();

        assert!(variables.functions_locals.is_empty());
        assert!(variables.queue.is_empty());
    }

    #[test]
    fn insert_local() {
        let mut variables = Variables::new();

        variables.insert_local("MAIN", "x");
        variables.insert_local("FUNC", "x");
        variables.insert_local("MAIN", "y");

        let mut expected_map = HashMap::new();
        expected_map.insert("MAIN", HashMap::new());
        expected_map.insert("FUNC", HashMap::new());
        expected_map.get_mut("MAIN").unwrap().insert("x", 0);
        expected_map.get_mut("FUNC").unwrap().insert("x", 0);
        expected_map.get_mut("MAIN").unwrap().insert("y", 1);

        let expected_queue = VecDeque::from([0, 0, 1]);

        assert_eq!(variables.functions_locals, expected_map);
        assert_eq!(variables.queue, expected_queue);
    }

    #[test]
    fn queue_pop_front() {
        let mut variables = Variables::new();

        variables.insert_local("MAIN", "x");
        variables.insert_local("FUNC", "x");
        variables.insert_local("MAIN", "y");

        let expected_queue = VecDeque::from([0, 1]);

        variables.queue_pop_front().unwrap();

        assert_eq!(variables.queue, expected_queue);
    }

    #[test]
    fn queue_pop_front_should_return_error_when_queue_empty() {
        let mut variables = Variables::new();

        assert!(variables.queue_pop_front().is_err());
    }
}
