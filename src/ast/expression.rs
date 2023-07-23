use super::statement::StatementNode;
use crate::token::{token_type::TokenType, Token};

#[derive(Debug, Clone)]
pub enum ExpressionNode {
    // None,
    Ident(Token),
    Int(Token),
    Bool(Token),
    UnaryOperator(Token, Box<ExpressionNode>),
    BinaryOperator(Token, Box<ExpressionNode>, Box<ExpressionNode>),
    Block(Vec<StatementNode>),
    If(Token, Box<ExpressionNode>, Block, Option<Block>),
    Function(Token, Vec<Ident>, Block),
    FunctionCall(Token, Box<ExpressionNode>, Vec<ExpressionNode>),
}

impl From<Ident> for ExpressionNode {
    fn from(value: Ident) -> Self {
        Self::Ident(value.0)
    }
}
impl From<Int> for ExpressionNode {
    fn from(value: Int) -> Self {
        Self::Int(value.0)
    }
}
impl From<Bool> for ExpressionNode {
    fn from(value: Bool) -> Self {
        Self::Bool(value.0)
    }
}
impl From<UnaryOperator> for ExpressionNode {
    fn from(value: UnaryOperator) -> Self {
        Self::UnaryOperator(value.token, Box::from(value.operand))
    }
}
impl From<BinaryOperator> for ExpressionNode {
    fn from(value: BinaryOperator) -> Self {
        Self::BinaryOperator(value.token, Box::from(value.lhs), Box::from(value.rhs))
    }
}
impl From<Block> for ExpressionNode {
    fn from(value: Block) -> Self {
        Self::Block(value.statements)
    }
}
impl From<If> for ExpressionNode {
    fn from(value: If) -> Self {
        Self::If(
            value.token,
            Box::from(value.condition),
            value.action,
            value.alternate,
        )
    }
}
impl From<Function> for ExpressionNode {
    fn from(value: Function) -> Self {
        Self::Function(value.token, value.args, value.body)
    }
}
impl From<FunctionCall> for ExpressionNode {
    fn from(value: FunctionCall) -> Self {
        Self::FunctionCall(value.token, Box::from(value.function), value.args)
    }
}
#[derive(Debug, Clone)]
pub struct Ident(Token);

impl Ident {
    pub fn new(inner: Token) -> Self {
        Self(inner)
    }

    pub fn get_string(&self) -> String {
        match &self.0.r#type {
            TokenType::Ident(ident) => ident.clone(),
            _ => panic!("Ident(AST) doesn't contain Ident(token)"),
        }
    }
}

impl From<Ident> for String {
    fn from(value: Ident) -> Self {
        value.get_string()
    }
}

#[derive(Debug)]
pub(crate) struct Int(Token);

impl Int {
    pub fn new(inner: Token) -> Self {
        Self(inner)
    }
}
#[derive(Debug)]
pub(crate) struct Bool(Token);

impl Bool {
    pub fn new(inner: Token) -> Self {
        Self(inner)
    }
}
#[derive(Debug)]
pub(crate) struct UnaryOperator {
    token: Token,
    operand: ExpressionNode,
}

impl UnaryOperator {
    pub fn new(operator: Token, operand: ExpressionNode) -> Self {
        Self {
            token: operator,
            operand,
        }
    }
}

#[derive(Debug)]
pub(crate) struct BinaryOperator {
    token: Token,
    lhs: ExpressionNode,
    rhs: ExpressionNode,
}

impl BinaryOperator {
    pub fn new(bop: Token, lhs: ExpressionNode, rhs: ExpressionNode) -> Self {
        Self {
            token: bop,
            lhs,
            rhs,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Block {
    pub statements: Vec<StatementNode>,
}

impl Block {
    pub fn new(statements: Vec<StatementNode>) -> Self {
        Self { statements }
    }
}

#[derive(Debug)]
pub(crate) struct If {
    token: Token,
    condition: ExpressionNode,
    action: Block,
    alternate: Option<Block>,
}

impl If {
    pub fn new(
        token: Token,
        condition: ExpressionNode,
        action: Block,
        alternate: Option<Block>,
    ) -> Self {
        Self {
            token,
            condition,
            action,
            alternate,
        }
    }
}

#[derive(Debug)]
pub(crate) struct Function {
    token: Token,
    args: Vec<Ident>,
    body: Block,
}

impl Function {
    pub fn new(token: Token, args: Vec<Ident>, body: Block) -> Self {
        Self { token, args, body }
    }
}

#[derive(Debug)]
pub(crate) struct FunctionCall {
    token: Token,
    function: ExpressionNode,
    args: Vec<ExpressionNode>,
}

impl FunctionCall {
    pub fn new(token: Token, function: ExpressionNode, args: Vec<ExpressionNode>) -> Self {
        Self {
            token,
            function,
            args,
        }
    }
}
