use super::env::Env;
use crate::ast::{expression::ExpressionNode, statement::StatementNode, Node};
use crate::object::Object;
use crate::token::{token_type::TokenType, Token};

#[allow(dead_code)]
#[derive(Debug)]
pub(super) struct EvalError {
    pub issue: String,
    pub token: Token,
}

fn eval_err(issue: String, token: Token) -> EvalError {
    EvalError { issue, token }
}

type EvalResult = Result<Object, EvalError>;

pub(super) struct Evaluator {
    env: Env,
}
impl Evaluator {
    pub fn new() -> Self {
        Self {
            env: Env::new(),
        }
    }
    pub fn eval(&mut self, node: Node) -> EvalResult {
        match node {
            Node::Program(program) => {
                let o = self.eval_block(program.statements)?;
                let o = if let Object::Return(o) = o { *o } else { o };
                Ok(o)
            }
            Node::Statement(stmt) => match stmt {
                StatementNode::Expression(expr) => self.eval(Node::Expression(expr)),
                StatementNode::Return(_, expr) => {
                    let ret_val = self.eval(Node::Expression(expr))?;
                    Ok(Object::Return(Box::from(ret_val)))
                }
                StatementNode::Let(_, ident, val) => {
                    let val = self.eval(Node::Expression(val))?;
                    self.env.set(ident.into(), &val); // irrecoverable error: problem with implementation
                    Ok(Object::Null)
                }
            },
            Node::Expression(expr) => match expr {
                ExpressionNode::Ident(token) => self.eval_ident(token),
                ExpressionNode::Int(token) => self.eval_int(token),
                ExpressionNode::Bool(token) => self.eval_bool(token),
                ExpressionNode::UnaryOperator(operator, operand) => {
                    let operand = self.eval(Node::Expression(*operand))?;
                    self.eval_unary(operator, operand)
                }
                ExpressionNode::BinaryOperator(operator, lhs, rhs) => {
                    let lhs = self.eval(Node::Expression(*lhs))?;
                    let rhs = self.eval(Node::Expression(*rhs))?;
                    self.eval_binary(operator, lhs, rhs)
                }
                ExpressionNode::Block(block) => self.eval_block(block),
                ExpressionNode::If(_, condition, action, alternate) => {
                    let condition: bool = self.eval(Node::Expression(*condition))?.into();
                    if condition {
                        self.eval_block(action.statements)
                    } else {
                        alternate.map_or(Ok(Object::Null), |alternate| {
                            self.eval_block(alternate.statements)
                        })
                    }
                }
                ExpressionNode::Function(token, args, block) => Ok(Object::Function(token, args, block)),
                ExpressionNode::FunctionCall(token, function, args) => {
                    let function = self.eval(Node::Expression(*function))?;
                    let args = args.into_iter().map(|arg| self.eval(Node::Expression(arg))).collect::<Result<Vec<Object>, EvalError>>()?;
                    if let Object::Function(token, params, block) = function {
                        if args.len() != params.len() {
                            return Err(eval_err(format!("Incorrect number of arguments passed, Got: {}, Expected: {}", args.len(), params.len()), token));
                        }

                        unimplemented!()
                    } else {
                        Err(eval_err("Can't call a non function".into(), token))
                    }
                }
                _ => unimplemented!(),
            },
        }
    }

    fn eval_ident(&self, token: Token) -> Result<Object, EvalError> {
        let key = token.clone().get_ident_name().unwrap(); // Ident(AST) contains Ident(Token)
        self.env.get(&key)
            .ok_or(eval_err("Failed to fetch the identifier".into(), token))
    }

    fn eval_int(&self, token: Token) -> EvalResult {
        if let TokenType::Int(ref int) = token.r#type {
            let int = int
                .parse::<isize>()
                .map_err(|err| eval_err(err.to_string(), token))?;
            Ok(Object::Int(int))
        } else {
            Err(eval_err("Expected Integer".into(), token))
        }
    }

    fn eval_bool(&self, token: Token) -> EvalResult {
        let o = match token.r#type {
            TokenType::True => Object::Bool(true),
            TokenType::False => Object::Bool(false),
            _ => return Err(eval_err("Expected boolean".into(), token)),
        };
        Ok(o)
    }

    fn eval_unary(&self, operator: Token, operand: Object) -> EvalResult {
        let o = match operator.r#type {
            TokenType::Bang => (!<Object as Into<bool>>::into(operand)).into(),
            TokenType::Plus => {
                if let Object::Int(int) = operand {
                    Object::Int(int)
                } else {
                    return Err(eval_err(
                        "Operand for the Unary Operator + should be an integer".into(),
                        operator,
                    ));
                }
            }
            TokenType::Minus => {
                if let Object::Int(int) = operand {
                    Object::Int(-1 * int)
                } else {
                    return Err(eval_err(
                        "Operand for the Unary Operator - should be an integer".into(),
                        operator,
                    ));
                }
            }
            _ => return Err(eval_err("Invalid unary operator".into(), operator)),
        };
        Ok(o)
    }

    fn eval_binary(&self, operator: Token, lhs: Object, rhs: Object) -> EvalResult {
        let o = match operator.r#type {
            TokenType::Plus     => (lhs + rhs).map_err(|err| eval_err(err, operator))?,
            TokenType::Minus    => (lhs - rhs).map_err(|err| eval_err(err, operator))?,
            TokenType::Asterisk => (lhs * rhs).map_err(|err| eval_err(err, operator))?,
            TokenType::Slash    => (lhs / rhs).map_err(|err| eval_err(err, operator))?,
            TokenType::Eq       => (lhs.eq(rhs)).map_err(|err| eval_err(err, operator))?,
            TokenType::NotEq    => (lhs.not_eq(rhs)).map_err(|err| eval_err(err, operator))?,
            TokenType::GT       => (lhs.gt(rhs)).map_err(|err| eval_err(err, operator))?,
            TokenType::LT       => (lhs.lt(rhs)).map_err(|err| eval_err(err, operator))?,
            _ => return Err(eval_err("Invalid binary operator".into(), operator)),
        };
        Ok(o)
    }

    // fn eval_program_inner(block: Vec<StatementNode>) -> Object {
    // block
    //     .into_iter()
    //     .map(|stmt| eval(Node::Statement(stmt)))
    //     .last()
    //     .unwrap_or(Object::Null)

    // reduce / fold?
    // early breaking is not possible
    // plus as executing statements is not pure / without side effects, i can't pull any tricks
    // like return || any = left
    // any || any = right
    // }

    fn eval_block(&mut self, block: Vec<StatementNode>) -> EvalResult {
        let mut result = None;
        for stmt in block {
            let r = self.eval(Node::Statement(stmt))?;
            if let Object::Return(_) = r {
                return Ok(r);
            }
            result = Some(r);
        }
        Ok(result.unwrap_or(Object::Null))
    }
}
