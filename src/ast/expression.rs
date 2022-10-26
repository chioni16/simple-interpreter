use crate::token::Token;
use super::statement::StatementNode;

#[derive(Debug)]
pub(crate) enum ExpressionNode {
    Ident(Token),
    Int(Token),
    Bool(Token),
    UnaryOperator(Token, Box<ExpressionNode>),
    BinaryOperator(Token, Box<ExpressionNode>, Box<ExpressionNode>),
    Block(Vec<StatementNode>),
    If(Box<ExpressionNode>, Block, Option<Block>),
    Function(Vec<Ident>, Block),
    FunctionCall(Box<ExpressionNode>, Vec<ExpressionNode>),
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
        Self::If(Box::from(value.condition), value.action, value.alternate)
    }
}
impl From<Function> for ExpressionNode {
    fn from(value: Function) -> Self {
        Self::Function(value.args, value.body)
    }
}
impl From<FunctionCall> for ExpressionNode {
    fn from(value: FunctionCall) -> Self {
        Self::FunctionCall(Box::from(value.name), value.args)
    }
}
#[derive(Debug)]
pub(crate) struct Ident(Token);


impl Ident {
    pub fn new(inner: Token) -> Self {
        Self(inner)
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

#[derive(Debug)]
pub(crate) struct Block {
    statements: Vec<StatementNode>,
}


impl Block {
    pub fn new(statements: Vec<StatementNode>) -> Self {
        Self {
            statements,
        }
    }
}

#[derive(Debug)]
pub(crate) struct If {
    condition: ExpressionNode,
    action: Block,
    alternate: Option<Block>,
}


impl If {
    pub fn new(condition: ExpressionNode, action: Block, alternate: Option<Block>) -> Self {
        Self {
            condition,
            action,
            alternate,
        }
    }
}


#[derive(Debug)]
pub(crate) struct Function {
    args: Vec<Ident>,
    body: Block,
}


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
    name: ExpressionNode,
    args: Vec<ExpressionNode>,
}

impl FunctionCall {
    pub fn new(name: ExpressionNode, args: Vec<ExpressionNode>) -> Self {
        Self {
            name,
            args,
        }
    }
}