use crate::ast::expression::ExpressionNode;
use crate::ast::statement::StatementNode;
use crate::ast::{Node, Program};
use crate::object::Object;
use crate::token::token_type::TokenType;

fn eval(node: Node) -> Object {
    match node {
        Node::Program(program) => {
            let o = eval_block(program.statements);
            if let Object::Return(o) = o {
                *o
            } else {
                o
            }
        }
        Node::Statement(stmt) => match stmt {
            StatementNode::Expression(expr) => eval(Node::Expression(expr)),
            StatementNode::Return(expr) => {
                let ret_val = eval(Node::Expression(expr));
                Object::Return(Box::from(ret_val))
            }
            _ => unimplemented!(),
        },
        Node::Expression(expr) => match expr {
            ExpressionNode::Int(token) => eval_int(token.r#type),
            ExpressionNode::Bool(token) => eval_bool(token.r#type),
            ExpressionNode::UnaryOperator(operator, operand) => {
                let operand = eval(Node::Expression(*operand));
                eval_unary(operator.r#type, operand)
            }
            ExpressionNode::BinaryOperator(operator, lhs, rhs) => {
                let lhs = eval(Node::Expression(*lhs));
                let rhs = eval(Node::Expression(*rhs));
                eval_binary(operator.r#type, lhs, rhs)
            }
            ExpressionNode::If(condition, action, alternate) => {
                let condition: bool = eval(Node::Expression(*condition)).into();
                if condition {
                    eval_block(action.statements)
                } else {
                    alternate.map_or(Object::Null, |alternate| eval_block(alternate.statements))
                }
            }
            _ => unimplemented!(),
        },
    }
}

fn eval_int(tt: TokenType) -> Object {
    if let TokenType::Int(int) = tt {
        Object::Int(int.parse().unwrap())
    } else {
        Object::Null
    }
}

fn eval_bool(tt: TokenType) -> Object {
    match tt {
        TokenType::True => Object::Bool(true),
        TokenType::False => Object::Bool(false),
        _ => Object::Null,
    }
}

fn eval_unary(operator: TokenType, operand: Object) -> Object {
    match operator {
        TokenType::Bang => match operand {
            Object::Null | Object::Bool(false) => Object::Bool(true),
            _ => Object::Bool(false),
        },
        TokenType::Plus => {
            if let Object::Int(int) = operand {
                Object::Int(int)
            } else {
                Object::Null
            }
        }
        TokenType::Minus => {
            if let Object::Int(int) = operand {
                Object::Int(-1 * int)
            } else {
                Object::Null
            }
        }
        _ => Object::Null,
    }
}

fn eval_binary(operator: TokenType, lhs: Object, rhs: Object) -> Object {
    match operator {
        TokenType::Plus => lhs + rhs,
        TokenType::Minus => lhs - rhs,
        TokenType::Asterisk => lhs * rhs,
        TokenType::Slash => lhs / rhs,
        TokenType::Eq => (lhs == rhs).into(),
        TokenType::NotEq => (lhs != rhs).into(),
        TokenType::GT => (lhs > rhs).into(),
        TokenType::LT => (lhs < rhs).into(),
        _ => Object::Null,
    }
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

fn eval_block(block: Vec<StatementNode>) -> Object {
    let mut result = None;
    for stmt in block {
        let r = eval(Node::Statement(stmt));
        if let Object::Return(_) = r {
            return r;
        }
        result = Some(r);
    }
    result.unwrap_or(Object::Null)
}

pub fn eval_program(program: Program) -> Object {
    eval(Node::Program(program))
}
