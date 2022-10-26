use std::ops::{Add, Sub, Mul, Div};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Object {
    Error(String),
    Return(Box<Object>),
    Int(isize),
    Bool(bool),
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