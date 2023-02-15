use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Clone)]
pub enum Object {
    Integer(i64),
    Float(f64),
    Nil,
}

impl Display for Object {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Self::Integer(int) => write!(f, "Integer({})", int),
            Self::Float(float) => write!(f, "Float({:.2})", float),
            Self::Nil => write!(f, "Nil"),
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
