use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Clone)]
pub enum Object {
    Number(f64),
    Nil,
}

impl Display for Object {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Self::Number(int) => write!(f, "{}", int),
            Self::Nil => write!(f, "nil"),
        }
    }
}

impl Object {
    pub fn is_truthy(&self) -> bool {
        match self {
            Self::Nil => false,
            _ => true,
        }
    }
}
