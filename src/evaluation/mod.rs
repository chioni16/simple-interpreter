use crate::ast::{Node, Program};
use crate::object::Object;

mod eval;
mod env;

use eval::Evaluator;

pub fn eval_program(program: Program) -> Object {
    let mut evaluator = Evaluator::new();
    evaluator.eval(Node::Program(program))
        .unwrap_or_else(|err| Object::Error(format!("{} @ {:?}", err.issue, err.token.span)))
}