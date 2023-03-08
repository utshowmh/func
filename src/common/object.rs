use std::fmt;

use super::{
    error::{Error, ErrorType},
    position::Position,
};

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Meta {
    pub is_return: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Object {
    Number(f64, Meta),
    String(String, Meta),
    Boolean(bool, Meta),
    Array(Vec<Object>, Meta),
    Nil(Meta),
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Number(number, ..) => write!(f, "{}", number),
            Self::String(string, ..) => write!(f, "{}", string.replace("\\n", "\n")),
            Self::Boolean(boolean, ..) => write!(f, "{}", boolean),
            Self::Array(array, ..) => {
                write!(f, "[")?;
                for object in array {
                    write!(f, "{},", object)?;
                }
                write!(f, "]")?;
                Ok(())
            }
            Self::Nil(..) => write!(f, "nil"),
        }
    }
}

impl Object {
    pub fn meta(&self) -> &Meta {
        match self {
            Self::Number(_, meta) => meta,
            Self::String(_, meta) => meta,
            Self::Boolean(_, meta) => meta,
            Self::Array(_, meta) => meta,
            Self::Nil(meta) => meta,
        }
    }

    pub fn set_return(&mut self) {
        match self {
            Self::Number(_, meta) => meta.is_return = true,
            Self::String(_, meta) => meta.is_return = true,
            Self::Boolean(_, meta) => meta.is_return = true,
            Self::Array(_, meta) => meta.is_return = true,
            Self::Nil(meta) => meta.is_return = true,
        }
    }

    pub fn is_return(&self) -> bool {
        match self {
            Self::Number(_, meta) => meta.is_return,
            Self::String(_, meta) => meta.is_return,
            Self::Boolean(_, meta) => meta.is_return,
            Self::Array(_, meta) => meta.is_return,
            Self::Nil(meta) => meta.is_return,
        }
    }

    pub fn is_truthy(&self) -> bool {
        match self {
            Self::Boolean(boolean, ..) => boolean.clone(),
            Self::Nil(..) => false,
            _ => true,
        }
    }

    pub fn push(&mut self, object: Object, position: Position) -> Result<Object, Error> {
        match self {
            Object::Array(array, ..) => {
                array.push(object);
                Ok(Object::Array(array.clone(), Meta::default()))
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
            Object::Array(array, ..) => {
                array.pop();
                Ok(Object::Array(array.clone(), Meta::default()))
            }
            _ => Err(Error::new(
                ErrorType::RuntimeError,
                format!("`{}` does not have `push` method associated with it", self),
                position,
            )),
        }
    }
}
