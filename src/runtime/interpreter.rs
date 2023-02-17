use crate::common::{
    ast::{
        BinaryExpression, BlockStatement, CallExpression, ElseBlock, Expression, FunctionStatement,
        GroupExpression, IdentifierExpression, IfStatement, LetStatement, PrintStatement, Program,
        Statement, UnaryExpression,
    },
    error::{Error, ErrorType},
    object::Object,
    token::TokenType,
};

use super::environment::{FunctionBindings, VariableBindings};

pub struct Interpreter {
    variables: VariableBindings,
    functions: FunctionBindings,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            variables: VariableBindings::new(),
            functions: FunctionBindings::new(),
        }
    }

    pub fn interpret(&mut self, program: Program) -> Result<(), Error> {
        for statement in program {
            self.execute_statement(statement)?;
        }
        Ok(())
    }

    fn execute_statement(&mut self, statement: Statement) -> Result<(), Error> {
        match statement {
            Statement::Let(let_statement) => self.execute_let_statement(let_statement),

            Statement::Function(function_statement) => {
                self.define_function_statement(function_statement)
            }

            Statement::If(if_statement) => self.execute_if_statement(if_statement),

            Statement::Print(print_statement) => self.execute_print_statement(print_statement),

            Statement::Block(block_statement) => self.execute_block_statement(block_statement),

            Statement::Expression(expression) => self.execute_expression(expression),
        }
    }

    fn execute_let_statement(&mut self, let_statement: LetStatement) -> Result<(), Error> {
        let identifier = let_statement.identifier;
        let value = self.evaluate_expression(let_statement.expression)?;
        self.variables.put(identifier, value);

        Ok(())
    }

    fn execute_if_statement(&mut self, if_statement: IfStatement) -> Result<(), Error> {
        let condition = self.evaluate_expression(if_statement.condition)?;
        if condition.is_truthy() {
            self.execute_block_statement(if_statement.if_block)?;
        } else {
            if let Some(else_block) = *if_statement.else_block {
                match else_block {
                    ElseBlock::Block(block_statment) => {
                        self.execute_block_statement(block_statment)?
                    }
                    ElseBlock::If(if_statement) => self.execute_if_statement(if_statement)?,
                }
            }
        }

        Ok(())
    }

    fn define_function_statement(
        &mut self,
        function_statement: FunctionStatement,
    ) -> Result<(), Error> {
        self.functions
            .put(function_statement.identifier.clone(), function_statement);
        Ok(())
    }

    fn execute_function_statement(
        &mut self,
        function_statement: FunctionStatement,
    ) -> Result<(), Error> {
        self.execute_block_statement(function_statement.block)
    }

    fn execute_print_statement(&mut self, print_statement: PrintStatement) -> Result<(), Error> {
        let value = self.evaluate_expression(print_statement.expression)?;
        println!("{}", value);

        Ok(())
    }

    fn execute_block_statement(&mut self, block_statment: BlockStatement) -> Result<(), Error> {
        let old_variables = self.variables.clone();
        for statement in *block_statment.statements {
            self.execute_statement(statement)?;
        }
        self.variables = old_variables;
        Ok(())
    }

    fn execute_expression(&mut self, expression: Expression) -> Result<(), Error> {
        self.evaluate_expression(expression)?;
        Ok(())
    }

    fn evaluate_expression(&mut self, expression: Expression) -> Result<Object, Error> {
        self.match_expression(expression)
    }

    fn evaluate_binary_expression(
        &mut self,
        binary_expression: BinaryExpression,
    ) -> Result<Object, Error> {
        let left = self.match_expression(*binary_expression.left)?;

        let right = self.match_expression(*binary_expression.right)?;

        match binary_expression.operator.ttype {
            TokenType::And => Ok(Object::Boolean(left.is_truthy() && right.is_truthy())),

            TokenType::Or => Ok(Object::Boolean(left.is_truthy() || right.is_truthy())),

            TokenType::EqualEqual => Ok(Object::Boolean(left == right)),

            TokenType::NotEqual => Ok(Object::Boolean(left != right)),

            TokenType::Greater => match (left, right) {
                (Object::Number(x), Object::Number(y)) => Ok(Object::Boolean(x > y)),

                (Object::Boolean(_), Object::Boolean(_)) => Err(Error::new(
                    ErrorType::RuntimeError,
                    format!(
                        "Type mismatch, `{}` doesn't support `boolean` as it's operand",
                        binary_expression.operator.lexeme
                    ),
                    binary_expression.operator.position,
                )),

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

            TokenType::GreaterEqual => match (left, right) {
                (Object::Number(x), Object::Number(y)) => Ok(Object::Boolean(x >= y)),

                (Object::Boolean(_), Object::Boolean(_)) => Err(Error::new(
                    ErrorType::RuntimeError,
                    format!(
                        "Type mismatch, `{}` doesn't support `boolean` as it's operand",
                        binary_expression.operator.lexeme
                    ),
                    binary_expression.operator.position,
                )),

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

            TokenType::Less => match (left, right) {
                (Object::Number(x), Object::Number(y)) => Ok(Object::Boolean(x < y)),

                (Object::Boolean(_), Object::Boolean(_)) => Err(Error::new(
                    ErrorType::RuntimeError,
                    format!(
                        "Type mismatch, `{}` doesn't support `boolean` as it's operand",
                        binary_expression.operator.lexeme
                    ),
                    binary_expression.operator.position,
                )),

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

            TokenType::LessEqual => match (left, right) {
                (Object::Number(x), Object::Number(y)) => Ok(Object::Boolean(x <= y)),

                (Object::Boolean(_), Object::Boolean(_)) => Err(Error::new(
                    ErrorType::RuntimeError,
                    format!(
                        "Type mismatch, `{}` doesn't support `boolean` as it's operand",
                        binary_expression.operator.lexeme
                    ),
                    binary_expression.operator.position,
                )),

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

            TokenType::Plus => match (left, right) {
                (Object::Number(x), Object::Number(y)) => Ok(Object::Number(x + y)),

                (Object::Boolean(_), Object::Boolean(_)) => Err(Error::new(
                    ErrorType::RuntimeError,
                    format!(
                        "Type mismatch, `{}` doesn't support `boolean` as it's operand",
                        binary_expression.operator.lexeme
                    ),
                    binary_expression.operator.position,
                )),

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

                (Object::Boolean(_), Object::Boolean(_)) => Err(Error::new(
                    ErrorType::RuntimeError,
                    format!(
                        "Type mismatch, `{}` doesn't support `boolean` as it's operand",
                        binary_expression.operator.lexeme
                    ),
                    binary_expression.operator.position,
                )),

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

                (Object::Boolean(_), Object::Boolean(_)) => Err(Error::new(
                    ErrorType::RuntimeError,
                    format!(
                        "Type mismatch, `{}` doesn't support `boolean` as it's operand",
                        binary_expression.operator.lexeme
                    ),
                    binary_expression.operator.position,
                )),

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

                (Object::Boolean(_), Object::Boolean(_)) => Err(Error::new(
                    ErrorType::RuntimeError,
                    format!(
                        "Type mismatch, `{}` doesn't support `boolean` as it's operand",
                        binary_expression.operator.lexeme
                    ),
                    binary_expression.operator.position,
                )),

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
        &mut self,
        unary_expression: UnaryExpression,
    ) -> Result<Object, Error> {
        let right = self.match_expression(*unary_expression.right)?;

        match unary_expression.operator.ttype {
            TokenType::Not => Ok(Object::Boolean(!right.is_truthy())),

            TokenType::Minus => match right {
                Object::Number(x) => Ok(Object::Number(x * -1.)),

                Object::Boolean(_) => Err(Error::new(
                    ErrorType::RuntimeError,
                    format!(
                        "Type mismatch, `{}` does not support `boolean` as it's operand",
                        unary_expression.operator.lexeme
                    ),
                    unary_expression.operator.position,
                )),

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
        &mut self,
        group_expression: GroupExpression,
    ) -> Result<Object, Error> {
        let value = self.evaluate_expression(*group_expression.child)?;
        Ok(value)
    }

    fn evaluate_call_expression(
        &mut self,
        call_expression: CallExpression,
    ) -> Result<Object, Error> {
        self.execute_function_statement(self.functions.get(call_expression.identifier)?)?;
        Ok(Object::Nil)
    }

    fn evaluate_identifier_expression(
        &self,
        identifier_expression: IdentifierExpression,
    ) -> Result<Object, Error> {
        self.variables.get(identifier_expression.identifier)
    }

    fn match_expression(&mut self, expression: Expression) -> Result<Object, Error> {
        match expression {
            Expression::Binary(binary_expression) => {
                self.evaluate_binary_expression(binary_expression)
            }

            Expression::Unary(unary_expression) => self.evaluate_unary_expression(unary_expression),

            Expression::Group(group_expression) => self.evaluate_group_expression(group_expression),

            Expression::Call(call_expression) => self.evaluate_call_expression(call_expression),

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
