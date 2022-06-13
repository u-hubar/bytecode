use std::collections::HashMap;

use crate::{stack::Stack, vm::Pointer};

pub type VariableAddress = usize;

#[derive(Debug)]
pub struct Variables<'buf> {
    functions_locals: HashMap<&'buf str, HashMap<&'buf str, Pointer>>,
    variables: Stack<VariableAddress>,
}

impl<'buf> Variables<'buf> {
    pub fn new() -> Self {
        Self {
            functions_locals: HashMap::new(),
            variables: Stack::new(),
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

        self.variables.push(*func_map.get(var_name).unwrap());
    }

    pub fn pop(&mut self) -> VariableAddress {
        self.variables.pop()
    }
}
