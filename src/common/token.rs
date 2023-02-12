use std::fmt::{Display, Formatter, Result};

use super::{object::Object, position::Position};

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    Identifier,
    Integer,
    Float,

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
            Self::Float => write!(f, "float"),
            Self::EOF => write!(f, "\0"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Token {
    pub ttype: TokenType,
    pub lexeme: String,
    pub literal: Option<Object>,
    pub position: Position,
}

impl Token {
    pub fn new(
        ttype: TokenType,
        lexeme: String,
        literal: Option<Object>,
        position: Position,
    ) -> Self {
        Self {
            ttype,
            lexeme,
            literal,
            position,
        }
    }
}
