#![feature(let_chains)]
#![feature(box_patterns)]

mod ast;
mod bytecode;
pub mod emitter;
mod env;
pub mod evaluation;
pub mod lexer;
mod object;
pub mod parser;
mod token;
// mod type_inference;
// mod typed_ast;
