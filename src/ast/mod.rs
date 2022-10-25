use std::fmt::Debug;

pub(crate) mod expression;
pub(crate) mod statement;

pub(crate) trait AstNode: Debug {}
pub(crate) trait StatementNode: AstNode {}
pub(crate) trait ExpressionNode: AstNode {}