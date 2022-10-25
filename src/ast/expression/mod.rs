use crate::token::Token;

use super::{ AstNode, ExpressionNode, StatementNode };

#[derive(Debug)]
pub(crate) struct Ident(Token);

impl AstNode for Ident {}
impl ExpressionNode for Ident {}

impl Ident {
    pub fn new(inner: Token) -> Self {
        Self(inner)
    }
}

#[derive(Debug)]
pub(crate) struct Int(Token);

impl AstNode for Int {}
impl ExpressionNode for Int {}

impl Int {
    pub fn new(inner: Token) -> Self {
        Self(inner)
    }
}
#[derive(Debug)]
pub(crate) struct Bool(Token);

impl AstNode for Bool {}
impl ExpressionNode for Bool {}

impl Bool {
    pub fn new(inner: Token) -> Self {
        Self(inner)
    }
}
#[derive(Debug)]
pub(crate) struct UnaryOperator {
    token: Token,
    operand: Box<dyn ExpressionNode>,
}

impl AstNode for UnaryOperator {}
impl ExpressionNode for UnaryOperator {}

impl UnaryOperator {
    pub fn new(operator: Token, operand: Box<dyn ExpressionNode>) -> Self {
        Self {
            token: operator,
            operand,
        }
    }
}

#[derive(Debug)]
pub(crate) struct BinaryOperator {
    token: Token,
    lhs: Box<dyn ExpressionNode>,
    rhs: Box<dyn ExpressionNode>,
}

impl AstNode for BinaryOperator {}
impl ExpressionNode for BinaryOperator {}

impl BinaryOperator {
    pub fn new(bop: Token, lhs: Box<dyn ExpressionNode>, rhs: Box<dyn ExpressionNode>) -> Self {
        Self {
            token: bop,
            lhs,
            rhs,
        }
    }
}

#[derive(Debug)]
pub(crate) struct Block {
    statements: Vec<Box<dyn StatementNode>>,
}

impl AstNode for Block {}
impl ExpressionNode for Block {}

impl Block {
    pub fn new(statements: Vec<Box<dyn StatementNode>>) -> Self {
        Self {
            statements,
        }
    }
}

#[derive(Debug)]
pub(crate) struct If {
    condition: Box<dyn ExpressionNode>,
    action: Block,
    alternate: Option<Block>,
}

impl AstNode for If {}
impl ExpressionNode for If {}

impl If {
    pub fn new(condition: Box<dyn ExpressionNode>, action: Block, alternate: Option<Block>) -> Self {
        Self {
            condition,
            action,
            alternate,
        }
    }
}


pub(crate) type ArgList = Vec<Ident>;

#[derive(Debug)]
pub(crate) struct Function {
    args: ArgList,
    body: Block,
}

impl AstNode for Function {}
impl ExpressionNode for Function {}

impl Function {
    pub fn new(args: Vec<Ident>, body: Block) -> Self {
        Self {
            args,
            body,
        }
    }
}

#[derive(Debug)]
pub(crate) struct FunctionCall {
    name: Box<dyn ExpressionNode>,
    args: Vec<Box<dyn ExpressionNode>>,
}
impl AstNode for FunctionCall {}
impl ExpressionNode for FunctionCall {}

impl FunctionCall {
    pub fn new(name: Box<dyn ExpressionNode>, args: Vec<Box<dyn ExpressionNode>>) -> Self {
        Self {
            name,
            args,
        }
    }
}