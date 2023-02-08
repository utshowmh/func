use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub enum Object {
    Integer(i64),
}

impl Display for Object {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Self::Integer(int) => write!(f, "{}", int),
        }
    }
}
