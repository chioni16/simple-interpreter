use super::expression::{ExpressionNode, Ident};

#[derive(Debug)]
pub(crate) enum StatementNode {
    Let(Ident, ExpressionNode),
    Return(ExpressionNode),
    Expression(ExpressionNode),
}

impl From<LetStatement> for StatementNode {
    fn from(value: LetStatement) -> Self {
        Self::Let(value.ident, value.assign_val)
    }
}

impl From<ReturnStatement> for StatementNode {
    fn from(value: ReturnStatement) -> Self {
        Self::Return(value.ret_val)
    }
}

impl From<ExpressionStatement> for StatementNode {
    fn from(value: ExpressionStatement) -> Self {
        Self::Expression(value.0)
    }
}

#[derive(Debug)]
pub(crate) struct LetStatement {
    ident: Ident,
    assign_val: ExpressionNode,
}

impl LetStatement {
    pub fn new(ident: Ident, assign_val: ExpressionNode) -> Self {
        Self { ident, assign_val }
    }
}

#[derive(Debug)]
pub(crate) struct ReturnStatement {
    ret_val: ExpressionNode,
}

impl ReturnStatement {
    pub(crate) fn new(ret_val: ExpressionNode) -> Self {
        Self { ret_val }
    }
}

#[derive(Debug)]
pub(crate) struct ExpressionStatement(ExpressionNode);

impl ExpressionStatement {
    pub(crate) fn new(expr: ExpressionNode) -> Self {
        Self(expr)
    }
}
