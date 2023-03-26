use std::cell::RefCell;
use std::rc::Rc;

use super::env::Env;
use crate::ast::{expression::ExpressionNode, statement::StatementNode, Node};
use crate::object::Object;
use crate::token::{token_type::TokenType, Token};

#[allow(dead_code)]
#[derive(Debug)]
pub(super) struct EvalError {
    pub issue: String,
    pub token: Token,
}

fn eval_err(issue: String, token: Token) -> EvalError {
    EvalError { issue, token }
}

type EvalResult = Result<Object, EvalError>;

pub(super) fn eval(node: Node, env: Rc<RefCell<Env>>) -> EvalResult {
    match node {
        Node::Program(program) => {
            let o = eval_block(program.statements, env.clone())?;
            let o = if let Object::Return(o) = o { *o } else { o };
            Ok(o)
        }
        Node::Statement(stmt) => match stmt {
            StatementNode::Expression(expr) => eval(Node::Expression(expr), env.clone()),
            StatementNode::Return(_, expr) => {
                let ret_val = eval(Node::Expression(expr), env.clone())?;
                Ok(Object::Return(Box::from(ret_val)))
            }
            StatementNode::Let(_, ident, val) => {
                let val = eval(Node::Expression(val), env.clone())?;
                env.borrow_mut().set(ident.into(), val); // irrecoverable error: problem with implementation
                Ok(Object::Null)
            }
        },
        Node::Expression(expr) => match expr {
            ExpressionNode::Ident(token) => eval_ident(token, env.clone()),
            ExpressionNode::Int(token) => eval_int(token),
            ExpressionNode::Bool(token) => eval_bool(token),
            ExpressionNode::UnaryOperator(operator, operand) => {
                let operand = eval(Node::Expression(*operand), env.clone())?;
                eval_unary(operator, operand)
            }
            ExpressionNode::BinaryOperator(operator, lhs, rhs) => {
                let lhs = eval(Node::Expression(*lhs), env.clone())?;
                let rhs = eval(Node::Expression(*rhs), env.clone())?;
                eval_binary(operator, lhs, rhs)
            }
            ExpressionNode::Block(block) => eval_block(block, env.clone()),
            ExpressionNode::If(_, condition, action, alternate) => {
                let condition: bool = eval(Node::Expression(*condition), env.clone())?.into();
                if condition {
                    eval_block(action.statements, env.clone())
                } else {
                    alternate.map_or(Ok(Object::Null), |alternate| {
                        eval_block(alternate.statements, env.clone())
                    })
                }
            }
            ExpressionNode::Function(token, args, block) => Ok(Object::Function(token, args, block, env.clone())),
            ExpressionNode::FunctionCall(token, function, args) => {
                let function = eval(Node::Expression(*function), env.clone())?;
                let args = args.into_iter().map(|arg| eval(Node::Expression(arg), env.clone())).collect::<Result<Vec<Object>, EvalError>>()?;
                if let Object::Function(token, params, block, env) = function {
                    if args.len() != params.len() {
                        return Err(eval_err(format!("Incorrect number of arguments passed, Got: {}, Expected: {}", args.len(), params.len()), token));
                    }

                    let new_env = Env::extend(env.clone());

                    for (p, a) in params.into_iter().map(|p| p.into()).zip(args.into_iter()) {
                        new_env.borrow_mut().set(p, a);
                    }

                    let ret_val =  eval_block(block.statements, new_env)?;
                    let ret_val = if let Object::Return(ret_val) = ret_val { *ret_val } else { ret_val };
                    Ok(ret_val)
                } else {
                    Err(eval_err("Can't call a non function".into(), token))
                }
            }
        },
    }
}

fn eval_ident(token: Token, env: Rc<RefCell<Env>>) -> Result<Object, EvalError> {
    let key = token.clone().get_ident_name().unwrap(); // Ident(AST) contains Ident(Token)
    env.borrow().get(&key)
        .ok_or(eval_err("Failed to fetch the identifier".into(), token))
}

fn eval_int(token: Token) -> EvalResult {
    if let TokenType::Int(ref int) = token.r#type {
        let int = int
            .parse::<isize>()
            .map_err(|err| eval_err(err.to_string(), token))?;
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
                return Err(eval_err(
                    "Operand for the Unary Operator + should be an integer".into(),
                    operator,
                ));
            }
        }
        TokenType::Minus => {
            if let Object::Int(int) = operand {
                Object::Int(-1 * int)
            } else {
                return Err(eval_err(
                    "Operand for the Unary Operator - should be an integer".into(),
                    operator,
                ));
            }
        }
        _ => return Err(eval_err("Invalid unary operator".into(), operator)),
    };
    Ok(o)
}

fn eval_binary(operator: Token, lhs: Object, rhs: Object) -> EvalResult {
    let o = match operator.r#type {
        TokenType::Plus     => (lhs + rhs).map_err(|err| eval_err(err, operator))?,
        TokenType::Minus    => (lhs - rhs).map_err(|err| eval_err(err, operator))?,
        TokenType::Asterisk => (lhs * rhs).map_err(|err| eval_err(err, operator))?,
        TokenType::Slash    => (lhs / rhs).map_err(|err| eval_err(err, operator))?,
        TokenType::Eq       => (lhs.eq(rhs)).map_err(|err| eval_err(err, operator))?,
        TokenType::NotEq    => (lhs.not_eq(rhs)).map_err(|err| eval_err(err, operator))?,
        TokenType::GT       => (lhs.gt(rhs)).map_err(|err| eval_err(err, operator))?,
        TokenType::LT       => (lhs.lt(rhs)).map_err(|err| eval_err(err, operator))?,
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

fn eval_block(block: Vec<StatementNode>, env: Rc<RefCell<Env>>) -> EvalResult {
    let mut result = Object::Null;
    for stmt in block {
        let r = eval(Node::Statement(stmt), env.clone())?;
        if matches!(r, Object::Return(_)) {
            return Ok(r);
        }
        result = r;
    }
    Ok(result)
}
