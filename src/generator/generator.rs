use crate::common::{
    ast::{
        BinaryExpression, Expression, GroupExpression, LetStatement, Program, Statement,
        UnaryExpression,
    },
    error::{Error, ErrorType},
    object::Object,
    token::TokenType,
};

pub struct Generator {}

impl Generator {
    pub fn generate(program: Program) -> Result<String, Error> {
        let mut code = String::new();

        for statement in program {
            match statement {
                Statement::Let(let_statement) => {
                    code.push_str(&generate_let_statement(let_statement)?)
                }
                Statement::ExpressionStatement(expression) => {
                    code.push_str(&generate_expression(expression)?)
                }
            }
        }

        let code = format!("int main() {{  {}return 0; }}", code);
        Ok(code)
    }
}

fn generate_let_statement(let_statement: LetStatement) -> Result<String, Error> {
    let identifier = let_statement.identifier.lexeme;
    let value = evaluate_expression(let_statement.expression)?;
    let mut value_type = String::new();
    match value {
        Object::Integer(_) => value_type.push_str("int"),
    }
    Ok(format!("{} {} = {};", value_type, identifier, value))
}

fn generate_expression(expression: Expression) -> Result<String, Error> {
    let value = evaluate_expression(expression)?;
    Ok(format!("{}", value))
}

fn evaluate_expression(expression: Expression) -> Result<Object, Error> {
    match expression {
        Expression::Binary(binary_expression) => evaluate_binary_expression(binary_expression),
        Expression::Unary(unary_expression) => evaluate_unary_expression(unary_expression),
        Expression::Group(group_expression) => evaluate_group_expression(group_expression),
        Expression::Literal(literal_expression) => {
            if let Some(object) = literal_expression.object.literal {
                Ok(object)
            } else {
                todo!("Gotta return Nil");
            }
        }
    }
}

fn evaluate_binary_expression(binary_expression: BinaryExpression) -> Result<Object, Error> {
    let left = match *binary_expression.left {
        Expression::Binary(binary_expression) => evaluate_binary_expression(binary_expression)?,
        Expression::Unary(unary_expression) => evaluate_unary_expression(unary_expression)?,
        Expression::Group(group_expression) => evaluate_group_expression(group_expression)?,
        Expression::Literal(literal_expression) => {
            if let Some(object) = literal_expression.object.literal {
                object
            } else {
                todo!("Gotta return Nil");
            }
        }
    };

    let right = match *binary_expression.right {
        Expression::Binary(binary_expression) => evaluate_binary_expression(binary_expression)?,
        Expression::Unary(unary_expression) => evaluate_unary_expression(unary_expression)?,
        Expression::Group(group_expression) => evaluate_group_expression(group_expression)?,
        Expression::Literal(literal_expression) => {
            if let Some(object) = literal_expression.object.literal {
                object
            } else {
                todo!("Gotta return Nil");
            }
        }
    };

    match binary_expression.operator.ttype {
        TokenType::Plus => match (left, right) {
            (Object::Integer(x), Object::Integer(y)) => Ok(Object::Integer(x + y)),
        },
        TokenType::Minus => match (left, right) {
            (Object::Integer(x), Object::Integer(y)) => Ok(Object::Integer(x - y)),
        },
        TokenType::Star => match (left, right) {
            (Object::Integer(x), Object::Integer(y)) => Ok(Object::Integer(x * y)),
        },
        TokenType::Slash => match (left, right) {
            (Object::Integer(x), Object::Integer(y)) => Ok(Object::Integer(x / y)),
        },
        TokenType::Modulo => match (left, right) {
            (Object::Integer(x), Object::Integer(y)) => Ok(Object::Integer(x % y)),
        },

        _ => Err(Error::new(
            ErrorType::CompilingError,
            format!(
                "`{}` is not a binary operator.",
                binary_expression.operator.lexeme
            ),
            binary_expression.operator.line,
        )),
    }
}

fn evaluate_unary_expression(unary_expression: UnaryExpression) -> Result<Object, Error> {
    let right = match *unary_expression.right {
        Expression::Binary(binary_expression) => evaluate_binary_expression(binary_expression)?,
        Expression::Unary(unary_expression) => evaluate_unary_expression(unary_expression)?,
        Expression::Group(group_expression) => evaluate_group_expression(group_expression)?,
        Expression::Literal(literal_expression) => {
            if let Some(object) = literal_expression.object.literal {
                object
            } else {
                todo!("Gotta return Nil");
            }
        }
    };

    match unary_expression.operator.ttype {
        TokenType::Minus => match right {
            Object::Integer(x) => Ok(Object::Integer(x * -1)),
        },

        _ => Err(Error::new(
            ErrorType::CompilingError,
            format!(
                "`{}` is not a unary operator.",
                unary_expression.operator.lexeme
            ),
            unary_expression.operator.line,
        )),
    }
}

fn evaluate_group_expression(group_expression: GroupExpression) -> Result<Object, Error> {
    let value = evaluate_expression(*group_expression.child)?;
    Ok(value)
}
