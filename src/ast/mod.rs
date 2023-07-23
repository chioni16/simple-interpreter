use self::{expression::ExpressionNode, statement::StatementNode};

pub(crate) mod expression;
pub(crate) mod statement;

pub(crate) enum Node {
    Program(Program),
    Statement(StatementNode),
    Expression(ExpressionNode),
}

#[derive(Debug)]
pub struct Program {
    pub(crate) statements: Vec<StatementNode>,
}

impl Program {
    pub(crate) fn new() -> Self {
        Self { statements: vec![] }
    }
    pub(crate) fn add_statement(&mut self, stmt: StatementNode) {
        self.statements.push(stmt);
    }
}
