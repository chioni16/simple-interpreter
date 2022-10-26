use super::expression::{ExpressionNode, Ident};
use crate::token::Token;

#[derive(Debug)]
pub(crate) enum StatementNode {
    Let(Token, Ident, ExpressionNode),
    Return(Token, ExpressionNode),
    Expression(ExpressionNode),
}

impl From<LetStatement> for StatementNode {
    fn from(value: LetStatement) -> Self {
        Self::Let(value.token, value.ident, value.assign_val)
    }
}

impl From<ReturnStatement> for StatementNode {
    fn from(value: ReturnStatement) -> Self {
        Self::Return(value.token, value.ret_val)
    }
}

impl From<ExpressionStatement> for StatementNode {
    fn from(value: ExpressionStatement) -> Self {
        Self::Expression(value.0)
    }
}

#[derive(Debug)]
pub(crate) struct LetStatement {
    token: Token,
    ident: Ident,
    assign_val: ExpressionNode,
}

impl LetStatement {
    pub fn new(token: Token, ident: Ident, assign_val: ExpressionNode) -> Self {
        Self { token, ident, assign_val }
    }
}

#[derive(Debug)]
pub(crate) struct ReturnStatement {
    token: Token,
    ret_val: ExpressionNode,
}

impl ReturnStatement {
    pub(crate) fn new(token: Token, ret_val: ExpressionNode) -> Self {
        Self { token, ret_val }
    }
}

#[derive(Debug)]
pub(crate) struct ExpressionStatement(ExpressionNode);

impl ExpressionStatement {
    pub(crate) fn new(expr: ExpressionNode) -> Self {
        Self(expr)
    }
}
