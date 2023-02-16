use crate::common::{
    ast::{
        BinaryExpression, BlockStatement, Expression, GroupExpression, IdentifierExpression,
        LetStatement, LiteralExpression, PrintStatement, Program, Statement, UnaryExpression,
    },
    error::{Error, ErrorType},
    token::{Token, TokenType},
};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Result<Program, Error> {
        let mut program = Vec::new();

        while !self.eof() {
            program.push(self.statemet()?);
        }

        Ok(program)
    }

    fn peek(&self) -> Token {
        self.tokens[self.current].clone()
    }

    fn eof(&self) -> bool {
        self.peek().ttype == TokenType::EOF
    }

    fn advance(&mut self) {
        if !self.eof() {
            self.current += 1;
        }
    }

    fn does_match(&self, expexted: &[TokenType]) -> bool {
        expexted.contains(&self.peek().ttype)
    }

    fn next_token(&mut self) -> Token {
        let token = self.peek();
        self.advance();
        token
    }

    fn eat(&mut self, ttype: TokenType) -> Result<Token, Error> {
        let token = self.peek().clone();
        if token.ttype == ttype {
            self.advance();
            Ok(token)
        } else {
            Err(Error::new(
                ErrorType::ParsingError,
                format!("Expected `{}`, found `{}`", ttype, token.ttype),
                token.position,
            ))
        }
    }

    fn statemet(&mut self) -> Result<Statement, Error> {
        match self.peek().ttype {
            TokenType::Let => Ok(Statement::Let(self.let_statement()?)),
            TokenType::Print => Ok(Statement::Print(self.print_statement()?)),
            TokenType::OpenCurly => Ok(Statement::Block(self.block_statement()?)),
            _ => Err(Error::new(
                ErrorType::ParsingError,
                format!("Expression is not used."),
                self.peek().position,
            )),
        }
    }

    fn let_statement(&mut self) -> Result<LetStatement, Error> {
        self.advance();
        let identifier = self.eat(TokenType::Identifier)?;
        self.eat(TokenType::Equal)?;
        let expression = self.expression()?;
        Ok(LetStatement::new(identifier, expression))
    }

    fn print_statement(&mut self) -> Result<PrintStatement, Error> {
        self.advance();
        self.eat(TokenType::OpenParen)?;
        let expression = self.expression()?;
        self.eat(TokenType::CloseParen)?;
        Ok(PrintStatement::new(expression))
    }

    fn block_statement(&mut self) -> Result<BlockStatement, Error> {
        self.advance();
        let mut statements = Vec::new();
        loop {
            if self.does_match(&[TokenType::CloseCurly]) || self.eof() {
                break;
            }
            statements.push(self.statemet()?);
        }
        self.eat(TokenType::CloseCurly)?;
        Ok(BlockStatement::new(statements))
    }

    fn expression(&mut self) -> Result<Expression, Error> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expression, Error> {
        let mut left = self.additive()?;

        while self.does_match(&[
            TokenType::EqualEqual,
            TokenType::NotEqual,
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.next_token();
            let right = self.additive()?;
            left = Expression::Binary(BinaryExpression::new(left, operator, right));
        }

        return Ok(left);
    }

    fn additive(&mut self) -> Result<Expression, Error> {
        let mut left = self.multiplicative()?;

        while self.does_match(&[TokenType::Plus, TokenType::Minus]) {
            let operator = self.next_token();
            let right = self.multiplicative()?;
            left = Expression::Binary(BinaryExpression::new(left, operator, right));
        }

        return Ok(left);
    }

    fn multiplicative(&mut self) -> Result<Expression, Error> {
        let mut left = self.unary()?;

        while self.does_match(&[TokenType::Star, TokenType::Slash, TokenType::Modulo]) {
            let operator = self.next_token();
            let right = self.unary()?;
            left = Expression::Binary(BinaryExpression::new(left, operator, right));
        }

        return Ok(left);
    }

    fn unary(&mut self) -> Result<Expression, Error> {
        while self.does_match(&[TokenType::Minus, TokenType::Not]) {
            let operator = self.next_token();
            let right = self.primary()?;
            return Ok(Expression::Unary(UnaryExpression::new(operator, right)));
        }
        self.primary()
    }

    fn primary(&mut self) -> Result<Expression, Error> {
        if self.does_match(&[
            TokenType::Number,
            TokenType::String,
            TokenType::Boolean,
            TokenType::Nil,
        ]) {
            Ok(Expression::Literal(LiteralExpression::new(
                self.next_token(),
            )))
        } else if self.does_match(&[TokenType::Identifier]) {
            Ok(Expression::Identifier(IdentifierExpression::new(
                self.next_token(),
            )))
        } else if self.does_match(&[TokenType::OpenParen]) {
            self.advance();
            let child = self.expression()?;
            self.eat(TokenType::CloseParen)?;
            Ok(Expression::Group(GroupExpression::new(child)))
        } else {
            let token = self.peek();
            Err(Error::new(
                ErrorType::ParsingError,
                format!("Unexpected token `{}`", token.ttype),
                token.position,
            ))
        }
    }
}
