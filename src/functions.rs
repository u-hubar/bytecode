use std::collections::HashMap;

use crate::{vm::Pointer, errors::ParseError};

#[derive(Debug)]
pub struct Functions(HashMap<String, (Pointer, Pointer)>);

impl Functions {
    pub fn new() -> Functions {
        Functions(HashMap::new())
    }

    pub fn insert(
        &mut self, func_name: String, (start_ip, end_ip): (Pointer, Pointer)
    ) -> Result<(), ParseError> {
        match self.0.insert(func_name, (start_ip, end_ip)) {
            Some(_) => Err(ParseError::DuplicatedFunction),
            None => Ok(()),
        }
    }

    pub fn get(&self, func_name: &str) -> Result<&(Pointer, Pointer), ParseError> {
        self.0.get(func_name).ok_or(ParseError::FunctionNotFound)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new() {
        let functions: Functions = Functions::new();

        assert!(functions.0.is_empty());
    }

    #[test]
    fn insert() {
        let mut functions: Functions = Functions::new();

        functions.insert("MAIN".to_string(), (1, 5)).unwrap();

        assert!(functions.0.contains_key("MAIN"));
    }

    #[test]
    fn insert_should_return_error_when_function_duplicated() {
        let mut functions: Functions = Functions::new();

        functions.insert("MAIN".to_string(), (1, 5)).unwrap();

        assert!(functions.insert("MAIN".to_string(), (1, 5)).is_err());
    }

    #[test]
    fn get() {
        let mut functions: Functions = Functions::new();

        functions.insert("MAIN".to_string(), (1, 5)).unwrap();

        assert_eq!(functions.get("MAIN").unwrap(), &(1, 5));
    }

    #[test]
    fn get_should_return_error_when_key_not_presented() {
        let functions: Functions = Functions::new();

        assert!(functions.get("MAIN").is_err());
    }
}
