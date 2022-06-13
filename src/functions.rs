use std::collections::HashMap;

use crate::vm::Pointer;

#[derive(Debug)]
pub struct Functions(HashMap<String, (Pointer, Pointer)>);

impl Functions {
    pub fn new() -> Functions {
        Functions(HashMap::new())
    }

    pub fn insert(&mut self, func_name: String, (start_ip, end_ip): (Pointer, Pointer)) {
        match self.0.insert(func_name, (start_ip, end_ip)) {
            Some(_) => panic!("Duplicated function!"),
            None => {},
        };
    }

    pub fn get(&self, func_name: &str) -> &(Pointer, Pointer) {
        self.0.get(func_name).expect("Function doesn't exist.")
    }
}
