use super::token::Token;

pub type Program = Vec<Statement>;

#[derive(Debug)]
pub enum Statement {
    Let(LetStatement),
    Print(PrintStatement),
    Block(BlockStatement),
    Expression(Expression),
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
pub struct PrintStatement {
    pub expression: Expression,
}

impl PrintStatement {
    pub fn new(expression: Expression) -> Self {
        Self { expression }
    }
}

#[derive(Debug)]
pub struct BlockStatement {
    pub statements: Box<Vec<Statement>>,
}

impl BlockStatement {
    pub fn new(statements: Vec<Statement>) -> Self {
        Self {
            statements: Box::new(statements),
        }
    }
}

#[derive(Debug)]
pub enum Expression {
    Binary(BinaryExpression),
    Unary(UnaryExpression),
    Group(GroupExpression),
    Identifier(IdentifierExpression),
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
pub struct IdentifierExpression {
    pub identifier: Token,
}

impl IdentifierExpression {
    pub fn new(identifier: Token) -> Self {
        Self { identifier }
    }
}

#[derive(Debug)]
pub struct LiteralExpression {
    pub object: Token,
}

impl LiteralExpression {
    pub fn new(object: Token) -> Self {
        Self { object }
    }
}
