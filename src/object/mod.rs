use std::cell::RefCell;
use std::ops::{Add, Sub, Mul, Div};
use std::rc::Rc;
use crate::ast;
use crate::token::Token;
use crate::evaluation::Env;

#[derive(Debug, Clone)]
pub enum Object {
    Error(String),
    Return(Box<Object>),
    Int(isize),
    Bool(bool),
    Function(Token, Vec<ast::expression::Ident>, ast::expression::Block, Rc<RefCell<Env>>),
    Null,
}

impl Add for Object {
    type Output = Result<Object, String>;
    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Object::Int(lhs), Object::Int(rhs)) => Ok(Object::Int(lhs+rhs)),
            // (Object::Bool(lhs), Object::Bool(rhs)) => Object::Int(lhs as isize + rhs as isize),
            _ => Err("Addition requires that both operands are integers".into()),
        } 
    }
}
impl Sub for Object {
    type Output = Result<Object, String>;
    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Object::Int(lhs), Object::Int(rhs)) => Ok(Object::Int(lhs-rhs)),
            _ => Err("Subtraction requires that both operands are integers".into()),
        } 
    }
}

impl Mul for Object {
    type Output = Result<Object, String>;
    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Object::Int(lhs), Object::Int(rhs)) => Ok(Object::Int(lhs*rhs)),
            _ => Err("Multiplication requires that both operands are integers".into()),
        } 
    }
}

impl Div for Object {
    type Output = Result<Object, String>;
    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Object::Int(lhs), Object::Int(rhs)) => Ok(Object::Int(lhs/rhs)),
            _ => Err("Division requires that both operands are integers".into()),
        } 
    }
}

impl Object {
    pub(crate) fn eq(self, rhs: Self) -> Result<Object, String> {
        match (&self, &rhs) {
            (Object::Null, Object::Null) => Ok(Object::Bool(true)),
            (Object::Int(one), Object::Int(two)) => Ok(Object::Bool(one == two)),
            (Object::Bool(one), Object::Bool(two)) => Ok(Object::Bool(one == two)),
            _ => Err(format!("==/!= operator is not valid for types: {:?}, {:?}", self, rhs))
        }
    }

    pub(crate) fn not_eq(self, rhs: Self) -> Result<Object, String> {
        let eq = self.eq(rhs)?;
        match eq {
            Object::Bool(false) => Ok(Object::Bool(true)),
            Object::Bool(true) => Ok(Object::Bool(false)),
            _ => unreachable!()
        }
    }
    
    pub(crate) fn gt(self, rhs: Self) -> Result<Object, String> {
        match (&self, &rhs) {
            (Object::Int(one), Object::Int(two)) => Ok(Object::Bool(one > two)),
            _ => Err(format!(">/< operator is not valid for types: {:?}, {:?}", self, rhs))
        }
    }

    pub(crate) fn lt(self, rhs: Self) -> Result<Object, String> {
        match (&self, &rhs) {
            (Object::Int(one), Object::Int(two)) => Ok(Object::Bool(one < two)),
            _ => Err(format!(">/< operator is not valid for types: {:?}, {:?}", self, rhs))
        }
    }
}

impl From<bool> for Object {
    fn from(value: bool) -> Self {
        Object::Bool(value)
    }
}

impl Into<bool> for Object {
    fn into(self) -> bool {
        match self {
            Object::Null | Object::Bool(false) => false,
            _                                  => true,
        }
    }
}