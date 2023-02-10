use std::fmt::{Display, Formatter, Result};

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
    line: usize,
}

impl Error {
    pub fn new(e_type: ErrorType, message: String, line: usize) -> Self {
        Self {
            e_type,
            message,
            line,
        }
    }

    pub fn report(&self) {
        eprintln!("{}: {} in line {}.", self.e_type, self.message, self.line);
    }
}
