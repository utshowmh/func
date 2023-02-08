use super::token::Token;

#[derive(Debug)]
pub enum Statement {
    ExpressionStatement(Expression),
}

#[derive(Debug)]
pub enum Expression {
    Binary(),
    Unary(),
    Group(),
    Literal(),
}

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

pub struct LiteralExpression {
    pub literal: Token,
}

impl LiteralExpression {
    pub fn new(literal: Token) -> Self {
        Self { literal }
    }
}
