use std::fmt::{Display, Formatter, Result};

use super::position::Position;

pub enum ErrorType {
    LexingError,
    ParsingError,
    RuntimeError,
}

impl Display for ErrorType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Self::LexingError => write!(f, "LexingError"),
            Self::ParsingError => write!(f, "ParsingError"),
            Self::RuntimeError => write!(f, "RuntimeError"),
        }
    }
}

pub struct Error {
    e_type: ErrorType,
    message: String,
    position: Position,
}

impl Error {
    pub fn new(e_type: ErrorType, message: String, position: Position) -> Self {
        Self {
            e_type,
            message,
            position,
        }
    }

    pub fn report(&self) {
        eprintln!(
            "{}: {} in line {}:{} (file: {}).",
            self.e_type,
            self.message,
            self.position.row,
            self.position.column,
            self.position.source_path
        );
    }
}
