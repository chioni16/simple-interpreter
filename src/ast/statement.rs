use super::{ AstNode, StatementNode, ExpressionNode };
use super::expression::Ident;

#[derive(Debug)]
pub struct Program {
    statements: Vec<Box<dyn StatementNode>>,    
}

impl AstNode for Program {}

impl Program {
    pub(crate) fn new() -> Self {
        Self { statements: vec![] }
    }
    pub(crate) fn add_statement(&mut self, stmt:Box<dyn StatementNode> ) {
        self.statements.push(stmt);
    } 
}

#[derive(Debug)]
pub(crate) struct LetStatement {
    ident: Ident,
    assign_val: Box<dyn ExpressionNode>,
}

impl AstNode for LetStatement {}
impl StatementNode for LetStatement {}

impl LetStatement {
    pub fn new(ident: Ident, assign_val: Box<dyn ExpressionNode>) -> Self {
        Self {
            ident,
            assign_val, 
        }
    }
}

#[derive(Debug)]
pub(crate) struct ReturnStatement {
    ret_val: Box<dyn ExpressionNode>,
}

impl ReturnStatement {
    pub(crate) fn new(ret_val: Box<dyn ExpressionNode>) -> Self {
        Self {
            ret_val, 
        }
    }
}

impl AstNode for ReturnStatement {}
impl StatementNode for ReturnStatement {}

#[derive(Debug)]
pub(crate) struct ExpressionStatement(Box<dyn ExpressionNode>);

impl ExpressionStatement {
    pub(crate) fn new(expr: Box<dyn ExpressionNode>) -> Self {
        Self (expr)
    } 
}

impl AstNode for ExpressionStatement {}
impl StatementNode for ExpressionStatement {}