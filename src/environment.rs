use crate::expr::LiteralValue;
use crate::lexer::Token;
use std::collections::HashMap;

pub struct Environment {
    values: HashMap<String, LiteralValue>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
        }
    }

    pub fn define(&mut self, name: String, value: LiteralValue) {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: &String) -> Result<LiteralValue, String> {
        match self.values.get(name) {
            Some(value) => Ok(value.clone()),
            None => Err(format!("Variable {} not declared yet!", name)),
        }
    }
}
