use std::fmt;

use super::{
    error::{Error, ErrorType},
    position::Position,
};

#[derive(Debug, Clone, PartialEq)]
pub enum Object {
    Number(f64),
    String(String),
    Boolean(bool),
    Array(Vec<Object>),
    Nil,
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Number(number) => write!(f, "{}", number),
            Self::String(string) => write!(f, "{}", string.replace("\\n", "\n")),
            Self::Boolean(boolean) => write!(f, "{}", boolean),
            Self::Array(array) => {
                write!(f, "[")?;
                for object in array {
                    write!(f, "{},", object)?;
                }
                write!(f, "]")?;
                Ok(())
            }
            Self::Nil => write!(f, "nil"),
        }
    }
}

impl Object {
    pub fn is_truthy(&self) -> bool {
        match self {
            Self::Boolean(boolean) => *boolean,
            Self::Nil => false,
            _ => true,
        }
    }

    pub fn push(&mut self, object: Object, position: Position) -> Result<Object, Error> {
        match self {
            Object::Array(array) => {
                array.push(object);
                Ok(Object::Array(array.clone()))
            }
            _ => Err(Error::new(
                ErrorType::RuntimeError,
                format!("`{}` does not have `push` method associated with it", self),
                position,
            )),
        }
    }

    pub fn pop(&mut self, position: Position) -> Result<Object, Error> {
        match self {
            Object::Array(array) => {
                array.pop();
                Ok(Object::Array(array.clone()))
            }
            _ => Err(Error::new(
                ErrorType::RuntimeError,
                format!("`{}` does not have `push` method associated with it", self),
                position,
            )),
        }
    }
}
