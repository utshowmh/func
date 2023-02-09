use super::token::Token;

pub type Program = Vec<Statement>;

#[derive(Debug)]
pub enum Statement {
    Let(LetStatement),
    ExpressionStatement(Expression),
}

#[derive(Debug)]
pub struct LetStatement {
    pub identifier: Token,
    pub expression: Expression,
}

impl LetStatement {
    pub fn new(identifier: Token, expression: Expression) -> Self {
        Self {
            identifier,
            expression,
        }
    }
}

#[derive(Debug)]
pub enum Expression {
    Binary(BinaryExpression),
    Unary(UnaryExpression),
    Group(GroupExpression),
    Literal(LiteralExpression),
}

#[derive(Debug)]
pub struct BinaryExpression {
    pub left: Box<Expression>,
    pub operator: Token,
    pub right: Box<Expression>,
}

impl BinaryExpression {
    pub fn new(left: Expression, operator: Token, right: Expression) -> Self {
        Self {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        }
    }
}

#[derive(Debug)]
pub struct UnaryExpression {
    pub operator: Token,
    pub right: Box<Expression>,
}

impl UnaryExpression {
    pub fn new(operator: Token, right: Expression) -> Self {
        Self {
            operator,
            right: Box::new(right),
        }
    }
}

#[derive(Debug)]
pub struct GroupExpression {
    pub child: Box<Expression>,
}

impl GroupExpression {
    pub fn new(child: Expression) -> Self {
        Self {
            child: Box::new(child),
        }
    }
}

#[derive(Debug)]
pub struct LiteralExpression {
    pub literal: Token,
}

impl LiteralExpression {
    pub fn new(literal: Token) -> Self {
        Self { literal }
    }
}
