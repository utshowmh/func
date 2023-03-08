use std::io::stdin;

use crate::common::{
    ast::{
        AssignmentStatement, BinaryExpression, BlockExpression, BuiltinFunction,
        BuiltinFunctionStatement, CallExpression, ElseBlock, Expression, FunctionStatement,
        GroupExpression, IdentifierExpression, IfExpression, LetStatement, Program, Statement,
        UnaryExpression,
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

            Statement::Assignment(assignment_statement) => {
                self.execute_assignment_statement(assignment_statement)
            }

            Statement::Function(function_statement) => {
                self.define_function_statement(function_statement)
            }

            Statement::BuiltinFunction(builtin_function_statement) => {
                self.execute_builtin_function_statement(builtin_function_statement)
            }

            Statement::Expression(expression) => self.execute_expression(expression),
            _ => Ok(()),
        }
    }

    fn execute_let_statement(&mut self, let_statement: LetStatement) -> Result<(), Error> {
        let identifier = let_statement.identifier;
        let value = self.evaluate_expression(let_statement.expression)?;
        self.variables.declare(identifier, value);

        Ok(())
    }

    fn execute_assignment_statement(
        &mut self,
        assignment_statement: AssignmentStatement,
    ) -> Result<(), Error> {
        let identifier = assignment_statement.identifier;
        self.variables.get(identifier.clone())?;
        let value = self.evaluate_expression(assignment_statement.expression)?;
        self.variables.assign(identifier, value)?;

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
        arguments: Vec<Expression>,
        function_statement: FunctionStatement,
    ) -> Result<Object, Error> {
        let old_variables = self.variables.clone();

        for index in 0..arguments.len() {
            let identifier = function_statement.paramiters[index].clone();
            let value = self.evaluate_expression(arguments[index].clone())?;
            self.variables.declare(identifier, value);
        }
        let return_value = self.evaluate_block_expression(function_statement.block)?;

        self.variables = old_variables;
        Ok(return_value)
    }

    fn execute_builtin_function_statement(
        &mut self,
        builtin_function_statement: BuiltinFunctionStatement,
    ) -> Result<(), Error> {
        match builtin_function_statement.builtin_function {
            BuiltinFunction::Read => {
                let identifier = match builtin_function_statement.arguments[0].clone() {
                    Expression::Identifier(identifier) => identifier.identifier,
                    _ => panic!(), // We're never reaching this because we're 'eating' identifier token in parser.
                };
                let mut value = String::new();
                stdin().read_line(&mut value).unwrap();
                self.variables
                    .assign(identifier, Object::String(value.trim().to_string()))?;
            }

            BuiltinFunction::Write => {
                for argument in builtin_function_statement.arguments {
                    print!("{}", self.evaluate_expression(argument)?);
                }
            }

            BuiltinFunction::Push => {
                let identifier = match builtin_function_statement.arguments[1].clone() {
                    Expression::Identifier(identifier) => identifier.identifier,
                    _ => panic!(), // We're never reaching this because we're 'eating' identifier token in parser.
                };
                let object =
                    self.evaluate_expression(builtin_function_statement.arguments[0].clone())?;
                let mut array = self.variables.get(identifier.clone())?;
                self.variables
                    .assign(identifier.clone(), array.push(object, identifier.position)?)?;
            }

            BuiltinFunction::Pop => {
                let identifier = match builtin_function_statement.arguments[0].clone() {
                    Expression::Identifier(identifier) => identifier.identifier,
                    _ => panic!(), // We're never reaching this because we're 'eating' identifier token in parser.
                };
                let mut array = self.variables.get(identifier.clone())?;
                self.variables
                    .assign(identifier.clone(), array.pop(identifier.position)?)?;
            }
        }

        Ok(())
    }

    fn evaluate_if_expression(&mut self, if_statement: IfExpression) -> Result<Object, Error> {
        let condition = self.evaluate_expression(*if_statement.condition)?;
        if condition.is_truthy() {
            self.evaluate_block_expression(if_statement.if_block)
        } else if let Some(else_block) = *if_statement.else_block {
            match else_block {
                ElseBlock::Block(block_statment) => self.evaluate_block_expression(block_statment),
                ElseBlock::If(if_statement) => self.evaluate_if_expression(if_statement),
            }
        } else {
            Ok(Object::Nil)
        }
    }

    fn evaluate_block_expression(
        &mut self,
        block_expression: BlockExpression,
    ) -> Result<Object, Error> {
        let old_variables = self.variables.clone();
        let mut return_value = Object::Nil;
        for statement in *block_expression.statements {
            if let Statement::Return(return_expression) = statement {
                return_value = self.evaluate_expression(return_expression)?;
                break;
            }
            self.execute_statement(statement)?;
        }
        self.variables = old_variables;
        Ok(return_value)
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

                Object::Array(_) => Err(Error::new(
                    ErrorType::RuntimeError,
                    format!(
                        "Type mismatch, `{}` does not support `array` as it's operand",
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
        let function_statement = self.functions.get(call_expression.identifier.clone())?;
        let paramiters = function_statement.paramiters.len();
        let arguments = call_expression.arguments.len();
        if paramiters != arguments {
            return Err(Error::new(
                ErrorType::RuntimeError,
                format!("Expected {} arguments, got {}", paramiters, arguments),
                call_expression.identifier.position,
            ));
        } else {
            self.execute_function_statement(call_expression.arguments, function_statement)
        }
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

            Expression::Block(block_expression) => {
                if let Ok(object) = self.evaluate_block_expression(block_expression) {
                    Ok(object)
                } else {
                    Ok(Object::Nil)
                }
            }

            Expression::If(if_expression) => self.evaluate_if_expression(if_expression),

            Expression::Literal(literal_expression) => {
                if let Some(object) = literal_expression.object.literal {
                    Ok(object)
                } else {
                    Ok(Object::Nil)
                }
            }

            Expression::Array(array_expression) => {
                let mut objects = Vec::new();
                for object in array_expression.objects {
                    if let Some(object) = object.literal {
                        objects.push(object)
                    }
                }
                Ok(Object::Array(objects))
            }
        }
    }
}
