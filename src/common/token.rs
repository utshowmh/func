use super::object::Object;

#[derive(Debug, Clone)]
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
}

#[derive(Debug)]
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
