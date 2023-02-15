use std::collections::HashMap;

use crate::common::{
    error::{Error, ErrorType},
    object::Object,
    token::Token,
};

pub struct Environment {
    bindings: HashMap<String, Object>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            bindings: HashMap::new(),
        }
    }

    pub fn put(&mut self, identifier: Token, value: Object) {
        self.bindings.insert(identifier.lexeme, value);
    }

    pub fn get(&self, identifier: Token) -> Result<Object, Error> {
        if let Some(value) = self.bindings.get(&identifier.lexeme) {
            Ok(value.clone())
        } else {
            Err(Error::new(
                ErrorType::RuntimeError,
                format!("Variable {} doesn't exist.", identifier.lexeme),
                identifier.position,
            ))
        }
    }
}
