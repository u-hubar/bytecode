use std::collections::{HashMap, VecDeque};

use crate::{vm::Pointer, errors::ParseError};

pub type VariableAddress = usize;

#[derive(Debug)]
pub struct Variables<'buf> {
    functions_locals: HashMap<&'buf str, HashMap<&'buf str, Pointer>>,
    variables: VecDeque<VariableAddress>,
}

impl<'buf> Variables<'buf> {
    pub fn new() -> Self {
        Self {
            functions_locals: HashMap::new(),
            variables: VecDeque::new(),
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

        self.variables.push_back(*func_map.get(var_name).unwrap());
    }

    pub fn pop_front(&mut self) -> Result<VariableAddress, ParseError> {
        self.variables.pop_front().ok_or(ParseError::VariableNotFound)
    }
}
