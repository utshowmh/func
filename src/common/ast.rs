use super::token::Token;

pub type Program = Vec<Statement>;

#[derive(Debug, Clone)]
pub enum Statement {
    Let(LetStatement),
    Assignment(AssignmentStatement),
    Block(BlockStatement),
    If(IfStatement),
    Function(FunctionStatement),
    BuiltinFunction(BuiltinFunctionStatement),
    Expression(Expression),
}

#[derive(Debug, Clone)]
pub enum BuiltinFunction {
    Read,
    Write,
    Push,
}

#[derive(Debug, Clone)]
pub enum ElseBlock {
    Block(BlockStatement),
    If(IfStatement),
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct AssignmentStatement {
    pub identifier: Token,
    pub expression: Expression,
}

impl AssignmentStatement {
    pub fn new(identifier: Token, expression: Expression) -> Self {
        Self {
            identifier,
            expression,
        }
    }
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct IfStatement {
    pub condition: Expression,
    pub if_block: BlockStatement,
    pub else_block: Box<Option<ElseBlock>>,
}

impl IfStatement {
    pub fn new(
        condition: Expression,
        if_block: BlockStatement,
        else_block: Option<ElseBlock>,
    ) -> Self {
        Self {
            condition,
            if_block,
            else_block: Box::new(else_block),
        }
    }
}

#[derive(Debug, Clone)]
pub struct FunctionStatement {
    pub identifier: Token,
    pub paramiters: Vec<Token>,
    pub block: BlockStatement,
    pub is_builtin: bool,
}

impl FunctionStatement {
    pub fn new(
        identifier: Token,
        paramiters: Vec<Token>,
        block: BlockStatement,
        is_builtin: bool,
    ) -> Self {
        Self {
            identifier,
            paramiters,
            block,
            is_builtin,
        }
    }
}

#[derive(Debug, Clone)]
pub struct BuiltinFunctionStatement {
    pub builtin_function: BuiltinFunction,
    pub arguments: Vec<Expression>,
}

impl BuiltinFunctionStatement {
    pub fn new(builtin_function: BuiltinFunction, arguments: Vec<Expression>) -> Self {
        Self {
            builtin_function,
            arguments,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Expression {
    Binary(BinaryExpression),
    Unary(UnaryExpression),
    Group(GroupExpression),
    Call(CallExpression),
    Identifier(IdentifierExpression),
    Literal(LiteralExpression),
    Array(ArrayExpression),
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct CallExpression {
    pub identifier: Token,
    pub arguments: Vec<Expression>,
}

impl CallExpression {
    pub fn new(identifier: Token, arguments: Vec<Expression>) -> Self {
        Self {
            identifier,
            arguments,
        }
    }
}

#[derive(Debug, Clone)]
pub struct IdentifierExpression {
    pub identifier: Token,
}

impl IdentifierExpression {
    pub fn new(identifier: Token) -> Self {
        Self { identifier }
    }
}

#[derive(Debug, Clone)]
pub struct LiteralExpression {
    pub object: Token,
}

impl LiteralExpression {
    pub fn new(object: Token) -> Self {
        Self { object }
    }
}

#[derive(Debug, Clone)]
pub struct ArrayExpression {
    pub objects: Vec<Token>,
}

impl ArrayExpression {
    pub fn new(objects: Vec<Token>) -> Self {
        Self { objects }
    }
}
