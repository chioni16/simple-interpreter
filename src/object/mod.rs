use std::ops::{Add, Sub, Mul, Div};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Object {
    Return(Box<Object>),
    Int(isize),
    Bool(bool),
    Null,
}

impl Add for Object {
    type Output = Object;
    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Object::Int(lhs), Object::Int(rhs)) => Object::Int(lhs+rhs),
            // (Object::Bool(lhs), Object::Bool(rhs)) => Object::Int(lhs as isize + rhs as isize),
            _ => Object::Null,
        } 
    }
}
impl Sub for Object {
    type Output = Object;
    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Object::Int(lhs), Object::Int(rhs)) => Object::Int(lhs-rhs),
            _ => Object::Null,
        } 
    }
}

impl Mul for Object {
    type Output = Object;
    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Object::Int(lhs), Object::Int(rhs)) => Object::Int(lhs*rhs),
            _ => Object::Null,
        } 
    }
}

impl Div for Object {
    type Output = Object;
    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Object::Int(lhs), Object::Int(rhs)) => Object::Int(lhs/rhs),
            _ => Object::Null,
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