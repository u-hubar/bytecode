use std::{collections::HashMap, iter::FromIterator};

#[derive(Debug)]
pub struct Variables(HashMap<String, usize>);

impl Variables {
    pub fn new() -> Variables {
        Variables(HashMap::new())
    }

    pub fn insert(&mut self, var_name: String, var_idx: usize) {
        self.0.insert(var_name, var_idx);
    }

    pub fn get(&self, var_name: &str) -> &usize {
        self.0.get(var_name).expect("Variable doesn't exist.")
    }
}

impl FromIterator<String> for Variables {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        let mut variables = HashMap::new();
        let mut var_idx = 0;

        for var_name in iter {
            if !variables.contains_key(&var_name) {
                variables.insert(var_name, var_idx);
                var_idx += 1;
            }
        }
        Self(variables)
    }
}
