use crate::ast::{Node, Program};
use crate::env::Env;
use crate::object::Object;

mod eval;

use eval::eval;
// pub(crate) use env::Env;

pub fn eval_program(program: Program) -> Object {
    let env = Env::new();
    eval(Node::Program(program), env)
        .unwrap_or_else(|err| Object::Error(format!("{} @ {:?}", err.issue, err.token.span)))
}
