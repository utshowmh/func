use crate::common::{
    ast::{
        ArrayExpression, AssignmentStatement, BinaryExpression, BlockStatement, BuiltinFunction,
        BuiltinFunctionStatement, CallExpression, ElseBlock, Expression, FunctionStatement,
        GroupExpression, IdentifierExpression, IfStatement, LetStatement, LiteralExpression,
        Program, Statement, UnaryExpression,
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

    // It can go wrong, so rebember where it's being used.
    fn peek_next(&self) -> Token {
        self.tokens[self.current + 1].clone()
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
            TokenType::Func => Ok(Statement::Function(self.function_statement()?)),
            TokenType::Let => Ok(Statement::Let(self.let_statement()?)),
            TokenType::If => Ok(Statement::If(self.if_statement()?)),
            TokenType::OpenCurly => Ok(Statement::Block(self.block_statement()?)),
            current_ttype => {
                if self.does_match(&[TokenType::Read, TokenType::Write]) {
                    Ok(Statement::BuiltinFunction(
                        self.builtin_function_statement()?,
                    ))
                } else if current_ttype == TokenType::Identifier
                    && self.peek_next().ttype == TokenType::Equal
                {
                    Ok(Statement::Assignment(self.assignment_statement()?))
                } else {
                    Ok(Statement::Expression(self.expression()?))
                }
            }
        }
    }

    fn let_statement(&mut self) -> Result<LetStatement, Error> {
        self.advance();
        let identifier = self.eat(TokenType::Identifier)?;
        self.eat(TokenType::Equal)?;
        let expression = self.expression()?;

        Ok(LetStatement::new(identifier, expression))
    }

    fn assignment_statement(&mut self) -> Result<AssignmentStatement, Error> {
        let identifier = self.eat(TokenType::Identifier)?;
        self.eat(TokenType::Equal)?;
        let expression = self.expression()?;

        Ok(AssignmentStatement::new(identifier, expression))
    }

    fn function_statement(&mut self) -> Result<FunctionStatement, Error> {
        self.advance();
        let identifier = self.eat(TokenType::Identifier)?;
        let mut paramiters = Vec::new();
        self.eat(TokenType::OpenParen)?;
        if !self.does_match(&[TokenType::CloseParen]) && !self.eof() {
            loop {
                paramiters.push(self.eat(TokenType::Identifier)?);
                if self.does_match(&[TokenType::Comma]) {
                    self.advance();
                } else {
                    break;
                }
            }
        }
        self.eat(TokenType::CloseParen)?;
        let block = self.block_statement()?;

        Ok(FunctionStatement::new(identifier, paramiters, block, false))
    }

    fn builtin_function_statement(&mut self) -> Result<BuiltinFunctionStatement, Error> {
        let func_type = self.next_token().ttype;
        let mut builtin_func = None;

        self.eat(TokenType::OpenParen)?;

        match func_type {
            TokenType::Read => {
                let expression = self.expression()?;
                self.eat(TokenType::Comma)?;
                let out_var = self.eat(TokenType::Identifier)?;
                builtin_func = Some(BuiltinFunctionStatement::new(
                    BuiltinFunction::Read,
                    vec![
                        expression,
                        Expression::Identifier(IdentifierExpression::new(out_var)),
                    ],
                ));
            }
            TokenType::Write => {
                let mut arguments = Vec::new();
                loop {
                    arguments.push(self.expression()?);
                    if self.does_match(&[TokenType::Comma]) {
                        self.advance();
                    } else {
                        break;
                    }
                }
                builtin_func = Some(BuiltinFunctionStatement::new(
                    BuiltinFunction::Write,
                    arguments,
                ));
            }
            _ => {
                return Err(Error::new(
                    ErrorType::ParsingError,
                    format!(""),
                    self.peek().position,
                ));
            }
        }

        self.eat(TokenType::CloseParen)?;
        Ok(builtin_func.unwrap())
    }

    fn if_statement(&mut self) -> Result<IfStatement, Error> {
        self.advance();
        let condition = self.expression()?;
        let if_block = self.block_statement()?;
        let mut else_block = None;
        while self.does_match(&[TokenType::Else]) {
            self.advance();
            if self.does_match(&[TokenType::If]) {
                else_block = Some(ElseBlock::If(self.if_statement()?));
            } else {
                else_block = Some(ElseBlock::Block(self.block_statement()?));
            }
        }

        Ok(IfStatement::new(condition, if_block, else_block))
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
        self.and()
    }

    fn and(&mut self) -> Result<Expression, Error> {
        let mut left = self.or()?;

        while self.does_match(&[TokenType::And]) {
            let operator = self.next_token();
            let right = self.or()?;
            left = Expression::Binary(BinaryExpression::new(left, operator, right));
        }

        return Ok(left);
    }

    fn or(&mut self) -> Result<Expression, Error> {
        let mut left = self.equality()?;

        while self.does_match(&[TokenType::Or]) {
            let operator = self.next_token();
            let right = self.equality()?;
            left = Expression::Binary(BinaryExpression::new(left, operator, right));
        }

        return Ok(left);
    }

    fn equality(&mut self) -> Result<Expression, Error> {
        let mut left = self.comparison()?;

        while self.does_match(&[TokenType::EqualEqual, TokenType::NotEqual]) {
            let operator = self.next_token();
            let right = self.comparison()?;
            left = Expression::Binary(BinaryExpression::new(left, operator, right));
        }

        return Ok(left);
    }

    fn comparison(&mut self) -> Result<Expression, Error> {
        let mut left = self.additive()?;

        while self.does_match(&[
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
            let identifier = self.next_token();
            if self.does_match(&[TokenType::OpenParen]) {
                self.advance();
                let mut arguments = Vec::new();
                if !self.does_match(&[TokenType::CloseParen]) {
                    loop {
                        arguments.push(self.expression()?);
                        if self.does_match(&[TokenType::Comma]) {
                            self.advance();
                        } else {
                            break;
                        }
                    }
                }
                self.eat(TokenType::CloseParen)?;
                Ok(Expression::Call(CallExpression::new(identifier, arguments)))
            } else {
                Ok(Expression::Identifier(IdentifierExpression::new(
                    identifier,
                )))
            }
        } else if self.does_match(&[TokenType::OpenBrack]) {
            self.advance();
            let mut objects = Vec::new();
            loop {
                objects.push(self.next_token());
                if self.does_match(&[TokenType::Comma]) {
                    self.advance();
                    continue;
                } else {
                    break;
                }
            }
            self.eat(TokenType::CloseBrack)?;
            Ok(Expression::Array(ArrayExpression::new(objects)))
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
