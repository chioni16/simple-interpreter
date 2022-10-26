use crate::ast::{Node, Program};
use crate::object::Object;

mod eval;
mod env;

use eval::eval;
use env::Env;

pub fn eval_program(program: Program) -> Object {
    let mut env = Env::new();
    eval(Node::Program(program), &mut env)
        .unwrap_or_else(|err| Object::Error(format!("{} @ {:?}", err.issue, err.token.span)))
}