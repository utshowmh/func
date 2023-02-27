use std::fmt::{Display, Formatter, Result};

use super::{object::Object, position::Position};

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    Identifier,
    Number,
    String,
    Boolean,
    Nil,

    Plus,
    Minus,
    Star,
    Slash,
    Modulo,

    Equal,
    EqualEqual,
    Not,
    NotEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    And,
    Or,

    Comma,

    OpenParen,
    CloseParen,
    OpenBrack,
    CloseBrack,
    OpenCurly,
    CloseCurly,

    Let,
    Func,
    If,
    Else,
    Print,

    EOF,
}

impl Display for TokenType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Self::Identifier => write!(f, "identifier"),
            Self::Number => write!(f, "number"),
            Self::String => write!(f, "string"),
            Self::Boolean => write!(f, "boolean"),
            Self::Nil => write!(f, "nil"),

            Self::Plus => write!(f, "+"),
            Self::Minus => write!(f, "-"),
            Self::Star => write!(f, "*"),
            Self::Slash => write!(f, "/"),
            Self::Modulo => write!(f, "%"),

            Self::Equal => write!(f, "="),
            Self::EqualEqual => write!(f, "=="),
            Self::Not => write!(f, "!"),
            Self::NotEqual => write!(f, "!="),
            Self::Greater => write!(f, ">"),
            Self::GreaterEqual => write!(f, ">="),
            Self::Less => write!(f, "<"),
            Self::LessEqual => write!(f, "<="),
            Self::And => write!(f, "&&"),
            Self::Or => write!(f, "||"),

            Self::Comma => write!(f, ","),

            Self::OpenParen => write!(f, "("),
            Self::CloseParen => write!(f, ")"),
            Self::OpenBrack => write!(f, "["),
            Self::CloseBrack => write!(f, "]"),
            Self::OpenCurly => write!(f, "{{"),
            Self::CloseCurly => write!(f, "}}"),

            Self::Let => write!(f, "let"),
            Self::Func => write!(f, "func"),
            Self::If => write!(f, "if"),
            Self::Else => write!(f, "else"),
            Self::Print => write!(f, "print"),

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
