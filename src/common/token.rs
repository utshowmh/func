use std::fmt::{Display, Formatter, Result};

use super::object::Object;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    Identifier,
    Integer,

    Plus,
    Minus,
    Star,
    Slash,
    Modulo,

    OpenParen,
    CloseParen,
    Equal,

    Let,

    EOF,
}

impl Display for TokenType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Self::Identifier => write!(f, "identifier"),
            Self::Integer => write!(f, "int"),
            Self::Plus => write!(f, "+"),
            Self::Minus => write!(f, "-"),
            Self::Star => write!(f, "*"),
            Self::Slash => write!(f, "/"),
            Self::Modulo => write!(f, "%"),
            Self::OpenParen => write!(f, "("),
            Self::CloseParen => write!(f, ")"),
            Self::Equal => write!(f, "="),
            Self::Let => write!(f, "let"),
            Self::EOF => write!(f, "\0"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Token {
    pub ttype: TokenType,
    pub lexeme: String,
    pub literal: Option<Object>,
    pub line: usize,
}

impl Token {
    pub fn new(ttype: TokenType, lexeme: String, literal: Option<Object>, line: usize) -> Self {
        Self {
            ttype,
            lexeme,
            literal,
            line,
        }
    }
}
