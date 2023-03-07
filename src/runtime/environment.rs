use std::collections::HashMap;

use crate::common::{
    ast::FunctionStatement,
    error::{Error, ErrorType},
    object::Object,
    token::Token,
};

#[derive(Debug, Clone, Default)]
pub struct VariableBindings {
    bindings: HashMap<String, Object>,
}

impl VariableBindings {
    pub fn declare(&mut self, identifier: Token, value: Object) {
        self.bindings.insert(identifier.lexeme, value);
    }

    pub fn get(&self, identifier: Token) -> Result<Object, Error> {
        if let Some(value) = self.bindings.get(&identifier.lexeme) {
            Ok(value.clone())
        } else {
            Err(Error::new(
                ErrorType::RuntimeError,
                format!("Variable `{}` doesn't exist.", identifier.lexeme),
                identifier.position,
            ))
        }
    }

    pub fn assign(&mut self, identifier: Token, value: Object) -> Result<(), Error> {
        if self.bindings.get(&identifier.lexeme).is_some() {
            self.declare(identifier, value);
            Ok(())
        } else {
            Err(Error::new(
                ErrorType::RuntimeError,
                format!("Variable `{}` doesn't exist.", identifier.lexeme),
                identifier.position,
            ))
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct FunctionBindings {
    bindings: HashMap<String, FunctionStatement>,
}

impl FunctionBindings {
    pub fn put(&mut self, identifier: Token, value: FunctionStatement) {
        self.bindings.insert(identifier.lexeme, value);
    }

    pub fn get(&self, identifier: Token) -> Result<FunctionStatement, Error> {
        if let Some(value) = self.bindings.get(&identifier.lexeme) {
            Ok(value.clone())
        } else {
            Err(Error::new(
                ErrorType::RuntimeError,
                format!("Function `{}` doesn't exist.", identifier.lexeme),
                identifier.position,
            ))
        }
    }
}
