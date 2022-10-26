use crate::ast::{Node, Program, statement::StatementNode, expression::ExpressionNode};
use crate::object::Object;
use crate::token::{Token, token_type::TokenType};

#[allow(dead_code)]
#[derive(Debug)]
struct EvalError {
    issue: String,
    token: Token,
}

fn eval_err(issue: String, token: Token) -> EvalError {
    EvalError { issue, token}
}

type EvalResult = Result<Object, EvalError>;

fn eval(node: Node) -> EvalResult {
    match node {
        Node::Program(program) => {
            let o = eval_block(program.statements)?;
            let o = if let Object::Return(o) = o {
                *o
            } else {
                o
            };
            Ok(o)
        }
        Node::Statement(stmt) => match stmt {
            StatementNode::Expression(expr) => eval(Node::Expression(expr)),
            StatementNode::Return(_, expr) => {
                let ret_val = eval(Node::Expression(expr))?;
                Ok(Object::Return(Box::from(ret_val)))
            }
            _ => unimplemented!(),
        },
        Node::Expression(expr) => match expr {
            ExpressionNode::Int(token) => eval_int(token),
            ExpressionNode::Bool(token) => eval_bool(token),
            ExpressionNode::UnaryOperator(operator, operand) => {
                let operand = eval(Node::Expression(*operand))?;
                eval_unary(operator, operand)
            }
            ExpressionNode::BinaryOperator(operator, lhs, rhs) => {
                let lhs = eval(Node::Expression(*lhs))?;
                let rhs = eval(Node::Expression(*rhs))?;
                eval_binary(operator, lhs, rhs)
            }
            ExpressionNode::If(_, condition, action, alternate) => {
                let condition: bool = eval(Node::Expression(*condition))?.into();
                if condition {
                    eval_block(action.statements)
                } else {
                    alternate.map_or(Ok(Object::Null), |alternate| eval_block(alternate.statements))
                }
            }
            _ => unimplemented!(),
        },
    }
}

fn eval_int(token: Token) -> EvalResult {
    if let TokenType::Int(ref int) = token.r#type {
        let int = int
            .parse::<isize>()
            .map_err(|err| eval_err( err.to_string(), token))?;
        Ok(Object::Int(int))
    } else {
        Err(eval_err("Expected Integer".into(), token))
    }
}

fn eval_bool(token: Token) -> EvalResult {
    let o = match token.r#type {
        TokenType::True => Object::Bool(true),
        TokenType::False => Object::Bool(false),
        _ => return Err(eval_err("Expected boolean".into(), token)),
    };
    Ok(o)
}

fn eval_unary(operator: Token, operand: Object) -> EvalResult {
    let o = match operator.r#type {
        TokenType::Bang => (!<Object as Into<bool>>::into(operand)).into(),
        TokenType::Plus => {
            if let Object::Int(int) = operand {
                Object::Int(int)
            } else {
                return Err(eval_err("Operand for the Unary Operator + should be an integer".into(), operator))
            }
        }
        TokenType::Minus => {
            if let Object::Int(int) = operand {
                Object::Int(-1 * int)
            } else {
                return Err(eval_err("Operand for the Unary Operator - should be an integer".into(), operator))
            }
        }
        _ => return Err(eval_err("Invalid unary operator".into(), operator)),
    };
    Ok(o)
}

fn eval_binary(operator: Token, lhs: Object, rhs: Object) -> EvalResult {
    let o = match operator.r#type {
        TokenType::Plus => (lhs + rhs).map_err(|err| eval_err(err, operator))?,
        TokenType::Minus => (lhs - rhs).map_err(|err| eval_err(err, operator))?,
        TokenType::Asterisk => (lhs * rhs).map_err(|err| eval_err(err, operator))?,
        TokenType::Slash => (lhs / rhs).map_err(|err| eval_err(err, operator))?,
        TokenType::Eq => (lhs == rhs).into(),
        TokenType::NotEq => (lhs != rhs).into(),
        TokenType::GT => (lhs > rhs).into(),
        TokenType::LT => (lhs < rhs).into(),
        _ => return Err(eval_err("Invalid binary operator".into(), operator)),
    };
    Ok(o)
}

// fn eval_program_inner(block: Vec<StatementNode>) -> Object {
// block
//     .into_iter()
//     .map(|stmt| eval(Node::Statement(stmt)))
//     .last()
//     .unwrap_or(Object::Null)

// reduce / fold?
// early breaking is not possible
// plus as executing statements is not pure / without side effects, i can't pull any tricks
// like return || any = left
// any || any = right
// }

fn eval_block(block: Vec<StatementNode>) -> EvalResult {
    let mut result = None;
    for stmt in block {
        let r = eval(Node::Statement(stmt))?;
        if let Object::Return(_) = r {
            return Ok(r);
        }
        result = Some(r);
    }
    Ok(result.unwrap_or(Object::Null))
}

pub fn eval_program(program: Program) -> Object {
    eval(Node::Program(program))
        .unwrap_or_else(|err| Object::Error(format!("{} @ {:?}", err.issue, err.token.span)))
}
