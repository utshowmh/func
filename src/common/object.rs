use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Clone, PartialEq)]
pub enum Object {
    Number(f64),
    String(String),
    Boolean(bool),
    Nil,
}

impl Display for Object {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Self::Number(number) => write!(f, "{}", number),
            Self::String(string) => write!(f, "{}", string),
            Self::Boolean(boolean) => write!(f, "{}", boolean),
            Self::Nil => write!(f, "nil"),
        }
    }
}

impl Object {
    pub fn is_truthy(&self) -> bool {
        match self {
            Self::Boolean(boolean) => boolean.clone(),
            Self::Nil => false,
            _ => true,
        }
    }
}
