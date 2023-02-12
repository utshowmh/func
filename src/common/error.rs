use std::fmt::{Display, Formatter, Result};

use super::position::Position;

pub enum ErrorType {
    LexingError,
    ParsingError,
    CompilingError,
}

impl Display for ErrorType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Self::LexingError => write!(f, "LexingError"),
            Self::ParsingError => write!(f, "ParsingError"),
            Self::CompilingError => write!(f, "CompilingError"),
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
            "{}: {} in line {}:{} on file {}.",
            self.e_type,
            self.message,
            self.position.column,
            self.position.row,
            self.position.source_path
        );
    }
}
