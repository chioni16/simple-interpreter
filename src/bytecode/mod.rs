use crate::object::Object;

const OP_CONSTANT: u8 = 1;
const OP_POP: u8 = 2;
const OP_NOT: u8 = 3;
const OP_UPLUS: u8 = 4;
const OP_UMIN: u8 = 5;
const OP_ADD: u8 = 6;
const OP_SUB: u8 = 7;
const OP_MUL: u8 = 8;
const OP_DIV: u8 = 9;
const OP_EQ: u8 = 10;
const OP_NEQ: u8 = 11;
const OP_GT: u8 = 12;
const OP_LT: u8 = 13;
const OP_J: u8 = 14;
const OP_JNT: u8 = 15;
const OP_SET_GLOBAL: u8 = 16;
const OP_GET_GLOBAL: u8 = 17;

#[derive(Debug)]
pub enum Instruction {
    Constant(u16),
    Pop,
    Not,
    UnaryPlus,
    UnaryMinus,
    Add,
    Sub,
    Mul,
    Div,
    Eq,
    NotEq,
    GT,
    LT,
    Jump(usize),
    JumpNotTruthy(usize),
    SetGlobal(u16),
    GetGlobal(u16),
}

impl Instruction {
    pub fn to_bytes(&self) -> Vec<u8> {
        match self {
            Self::Constant(index) => {
                let mut v = vec![OP_CONSTANT];
                v.extend(index.to_be_bytes());
                v
            }
            Self::Pop => vec![OP_POP],
            Self::Not => vec![OP_NOT],
            Self::UnaryPlus => vec![OP_UPLUS],
            Self::UnaryMinus => vec![OP_UMIN],
            Self::Add => vec![OP_ADD],
            Self::Sub => vec![OP_SUB],
            Self::Mul => vec![OP_MUL],
            Self::Div => vec![OP_DIV],
            Self::Eq => vec![OP_EQ],
            Self::NotEq => vec![OP_NEQ],
            Self::GT => vec![OP_GT],
            Self::LT => vec![OP_LT],
            Self::Jump(loc) => {
                let mut v = vec![OP_J];
                v.extend(loc.to_be_bytes());
                v
            }
            Self::JumpNotTruthy(loc) => {
                let mut v = vec![OP_JNT];
                v.extend(loc.to_be_bytes());
                v
            }
            Self::SetGlobal(index) => {
                let mut v = vec![OP_SET_GLOBAL];
                v.extend(index.to_be_bytes());
                v
            }
            Self::GetGlobal(index) => {
                let mut v = vec![OP_GET_GLOBAL];
                v.extend(index.to_be_bytes());
                v
            }
        }
    }
}

#[derive(Debug)]
pub struct Bytecode {
    pub(crate) instructions: Vec<u8>,
    pub(crate) constants: Vec<Object>,
}
