use crate::common::{
    ast::{
        BinaryExpression, Expression, GroupExpression, IdentifierExpression, LetStatement,
        PrintStatement, Program, Statement, UnaryExpression,
    },
    error::{Error, ErrorType},
    object::Object,
    token::TokenType,
};

use super::environment::Environment;

pub struct Interpreter {
    environment: Environment,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            environment: Environment::new(),
        }
    }

    pub fn interpret(&mut self, program: Program) -> Result<(), Error> {
        for statement in program {
            match statement {
                Statement::Let(let_statement) => self.execute_let_statement(let_statement)?,
                Statement::Print(print_statement) => {
                    self.execute_print_statement(print_statement)?
                }
            }
        }

        Ok(())
    }

    fn execute_let_statement(&mut self, let_statement: LetStatement) -> Result<(), Error> {
        let identifier = let_statement.identifier;
        let value = self.evaluate_expression(let_statement.expression)?;
        self.environment.put(identifier, value);

        Ok(())
    }

    fn execute_print_statement(&self, print_statement: PrintStatement) -> Result<(), Error> {
        let value = self.evaluate_expression(print_statement.expression)?;
        println!("{}", value);

        Ok(())
    }

    fn evaluate_expression(&self, expression: Expression) -> Result<Object, Error> {
        self.match_expression(expression)
    }

    fn evaluate_binary_expression(
        &self,
        binary_expression: BinaryExpression,
    ) -> Result<Object, Error> {
        let left = self.match_expression(*binary_expression.left)?;

        let right = self.match_expression(*binary_expression.right)?;

        match binary_expression.operator.ttype {
            TokenType::Plus => match (left, right) {
                (Object::Number(x), Object::Number(y)) => Ok(Object::Number(x + y)),

                (Object::String(x), Object::String(y)) => Ok(Object::String(x + &y)),

                (Object::Nil, Object::Nil) => Err(Error::new(
                    ErrorType::RuntimeError,
                    format!(
                        "Type mismatch, `{}` doesn't support `nil` as it's operand",
                        binary_expression.operator.lexeme
                    ),
                    binary_expression.operator.position,
                )),

                _ => Err(Error::new(
                    ErrorType::RuntimeError,
                    format!(
                        "Type mismatch, `{}` expects same type on both side",
                        binary_expression.operator.lexeme
                    ),
                    binary_expression.operator.position,
                )),
            },

            TokenType::Minus => match (left, right) {
                (Object::Number(x), Object::Number(y)) => Ok(Object::Number(x - y)),

                (Object::String(_), Object::String(_)) => Err(Error::new(
                    ErrorType::RuntimeError,
                    format!(
                        "Type mismatch, `{}` doesn't support `string` as it's operand",
                        binary_expression.operator.lexeme
                    ),
                    binary_expression.operator.position,
                )),

                (Object::Nil, Object::Nil) => Err(Error::new(
                    ErrorType::RuntimeError,
                    format!(
                        "Type mismatch, `{}` doesn't support `nil` as it's operand",
                        binary_expression.operator.lexeme
                    ),
                    binary_expression.operator.position,
                )),

                _ => Err(Error::new(
                    ErrorType::RuntimeError,
                    format!(
                        "Type mismatch, `{}` expects same type on both side",
                        binary_expression.operator.lexeme
                    ),
                    binary_expression.operator.position,
                )),
            },

            TokenType::Star => match (left, right) {
                (Object::Number(x), Object::Number(y)) => Ok(Object::Number(x * y)),

                (Object::String(_), Object::String(_)) => Err(Error::new(
                    ErrorType::RuntimeError,
                    format!(
                        "Type mismatch, `{}` doesn't support `string` as it's operand",
                        binary_expression.operator.lexeme
                    ),
                    binary_expression.operator.position,
                )),

                (Object::Nil, Object::Nil) => Err(Error::new(
                    ErrorType::RuntimeError,
                    format!(
                        "Type mismatch, `{}` doesn't support `nil` as it's operand",
                        binary_expression.operator.lexeme
                    ),
                    binary_expression.operator.position,
                )),

                _ => Err(Error::new(
                    ErrorType::RuntimeError,
                    format!(
                        "Type mismatch, `{}` expects same type on both side",
                        binary_expression.operator.lexeme
                    ),
                    binary_expression.operator.position,
                )),
            },

            TokenType::Slash => match (left, right) {
                (Object::Number(x), Object::Number(y)) => Ok(Object::Number(x / y)),

                (Object::String(_), Object::String(_)) => Err(Error::new(
                    ErrorType::RuntimeError,
                    format!(
                        "Type mismatch, `{}` doesn't support `string` as it's operand",
                        binary_expression.operator.lexeme
                    ),
                    binary_expression.operator.position,
                )),

                (Object::Nil, Object::Nil) => Err(Error::new(
                    ErrorType::RuntimeError,
                    format!(
                        "Type mismatch, `{}` doesn't support `nil` as it's operand",
                        binary_expression.operator.lexeme
                    ),
                    binary_expression.operator.position,
                )),

                _ => Err(Error::new(
                    ErrorType::RuntimeError,
                    format!(
                        "Type mismatch, `{}` expects same type on both side",
                        binary_expression.operator.lexeme
                    ),
                    binary_expression.operator.position,
                )),
            },

            TokenType::Modulo => match (left, right) {
                (Object::Number(x), Object::Number(y)) => Ok(Object::Number(x % y)),

                (Object::String(_), Object::String(_)) => Err(Error::new(
                    ErrorType::RuntimeError,
                    format!(
                        "Type mismatch, `{}` doesn't support `string` as it's operand",
                        binary_expression.operator.lexeme
                    ),
                    binary_expression.operator.position,
                )),

                (Object::Nil, Object::Nil) => Err(Error::new(
                    ErrorType::RuntimeError,
                    format!(
                        "Type mismatch, `{}` doesn't support `nil` as it's operand",
                        binary_expression.operator.lexeme
                    ),
                    binary_expression.operator.position,
                )),

                _ => Err(Error::new(
                    ErrorType::RuntimeError,
                    format!(
                        "Type mismatch, `{}` expects same type on both side",
                        binary_expression.operator.lexeme
                    ),
                    binary_expression.operator.position,
                )),
            },

            _ => Err(Error::new(
                ErrorType::RuntimeError,
                format!(
                    "`{}` is not a binary operator.",
                    binary_expression.operator.lexeme
                ),
                binary_expression.operator.position,
            )),
        }
    }

    fn evaluate_unary_expression(
        &self,
        unary_expression: UnaryExpression,
    ) -> Result<Object, Error> {
        let right = self.match_expression(*unary_expression.right)?;

        match unary_expression.operator.ttype {
            TokenType::Minus => match right {
                Object::Number(x) => Ok(Object::Number(x * -1.)),
                Object::String(_) => Err(Error::new(
                    ErrorType::RuntimeError,
                    format!(
                        "Type mismatch, `{}` does not support `string` as it's operand",
                        unary_expression.operator.lexeme
                    ),
                    unary_expression.operator.position,
                )),
                Object::Nil => Err(Error::new(
                    ErrorType::RuntimeError,
                    format!(
                        "Type mismatch, `{}` does not support `nil` as it's operand",
                        unary_expression.operator.lexeme
                    ),
                    unary_expression.operator.position,
                )),
            },

            _ => Err(Error::new(
                ErrorType::RuntimeError,
                format!(
                    "`{}` is not a unary operator.",
                    unary_expression.operator.lexeme
                ),
                unary_expression.operator.position,
            )),
        }
    }

    fn evaluate_group_expression(
        &self,
        group_expression: GroupExpression,
    ) -> Result<Object, Error> {
        let value = self.evaluate_expression(*group_expression.child)?;
        Ok(value)
    }

    fn evaluate_identifier_expression(
        &self,
        identifier_expression: IdentifierExpression,
    ) -> Result<Object, Error> {
        self.environment.get(identifier_expression.identifier)
    }

    fn match_expression(&self, expression: Expression) -> Result<Object, Error> {
        match expression {
            Expression::Binary(binary_expression) => {
                self.evaluate_binary_expression(binary_expression)
            }
            Expression::Unary(unary_expression) => self.evaluate_unary_expression(unary_expression),
            Expression::Group(group_expression) => self.evaluate_group_expression(group_expression),
            Expression::Identifier(identifier_expression) => {
                Ok(self.evaluate_identifier_expression(identifier_expression)?)
            }
            Expression::Literal(literal_expression) => {
                if let Some(object) = literal_expression.object.literal {
                    Ok(object)
                } else {
                    Ok(Object::Nil)
                }
            }
        }
    }
}
