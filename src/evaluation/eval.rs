use crate::ast::{Node, statement::StatementNode, expression::ExpressionNode};
use crate::object::Object;
use crate::token::{Token, token_type::TokenType};
use super::env::Env;

#[allow(dead_code)]
#[derive(Debug)]
pub(super) struct EvalError {
    pub issue: String,
    pub token: Token,
}

fn eval_err(issue: String, token: Token) -> EvalError {
    EvalError { issue, token}
}

type EvalResult = Result<Object, EvalError>;

pub(super) fn eval(node: Node, env: &mut Env) -> EvalResult {
    match node {
        Node::Program(program) => {
            let o = eval_block(program.statements, env)?;
            let o = if let Object::Return(o) = o {
                *o
            } else {
                o
            };
            Ok(o)
        }
        Node::Statement(stmt) => match stmt {
            StatementNode::Expression(expr) => eval(Node::Expression(expr), env),
            StatementNode::Return(_, expr) => {
                let ret_val = eval(Node::Expression(expr), env)?;
                Ok(Object::Return(Box::from(ret_val)))
            }
            StatementNode::Let(_, ident, val) => {
                let val = eval(Node::Expression(val), env)?;
                env.set(ident.into(), &val); // irrecoverable error: problem with implementation
                Ok(Object::Null)
            }
        },
        Node::Expression(expr) => match expr {
            ExpressionNode::Ident(token) => eval_ident(token, env),
            ExpressionNode::Int(token) => eval_int(token, env),
            ExpressionNode::Bool(token) => eval_bool(token, env),
            ExpressionNode::UnaryOperator(operator, operand) => {
                let operand = eval(Node::Expression(*operand), env)?;
                eval_unary(operator, operand, env)
            }
            ExpressionNode::BinaryOperator(operator, lhs, rhs) => {
                let lhs = eval(Node::Expression(*lhs), env)?;
                let rhs = eval(Node::Expression(*rhs), env)?;
                eval_binary(operator, lhs, rhs, env)
            }
            ExpressionNode::If(_, condition, action, alternate) => {
                let condition: bool = eval(Node::Expression(*condition), env)?.into();
                if condition {
                    eval_block(action.statements, env)
                } else {
                    alternate.map_or(Ok(Object::Null), |alternate| eval_block(alternate.statements, env))
                }
            }
            _ => unimplemented!(),
        },
    }
}

fn eval_ident(token: Token, env: &mut Env) -> Result<Object, EvalError> {
    let key = token.clone().get_ident_name().unwrap(); // Ident(AST) contains Ident(Token)
    env.get(&key).ok_or(eval_err("Failed to fetch the identifier".into(), token))
}

fn eval_int(token: Token, _env: &mut Env) -> EvalResult {
    if let TokenType::Int(ref int) = token.r#type {
        let int = int
            .parse::<isize>()
            .map_err(|err| eval_err( err.to_string(), token))?;
        Ok(Object::Int(int))
    } else {
        Err(eval_err("Expected Integer".into(), token))
    }
}

fn eval_bool(token: Token, _env: &mut Env) -> EvalResult {
    let o = match token.r#type {
        TokenType::True => Object::Bool(true),
        TokenType::False => Object::Bool(false),
        _ => return Err(eval_err("Expected boolean".into(), token)),
    };
    Ok(o)
}

fn eval_unary(operator: Token, operand: Object, _env: &mut Env) -> EvalResult {
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

fn eval_binary(operator: Token, lhs: Object, rhs: Object, _env: &mut Env) -> EvalResult {
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

fn eval_block(block: Vec<StatementNode>, env: &mut Env) -> EvalResult {
    let mut result = None;
    for stmt in block {
        let r = eval(Node::Statement(stmt), env)?;
        if let Object::Return(_) = r {
            return Ok(r);
        }
        result = Some(r);
    }
    Ok(result.unwrap_or(Object::Null))
}


