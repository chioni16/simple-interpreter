use std::collections::HashMap;

use crate::ast::{expression::ExpressionNode, statement::StatementNode, Node, Program};
use crate::bytecode::{Bytecode, Instruction};
use crate::object::Object;
use crate::token::token_type::TokenType;

#[derive(Debug, Default, Clone, Copy)]
enum Scope {
    #[default]
    Global,
}

#[derive(Debug, Default, Clone)]
struct Symbol {
    name: String,
    scope: Scope,
    index: u16,
}

#[derive(Debug, Default)]
struct SymbolTable {
    store: HashMap<String, Symbol>,
    num_definitions: u16,
}

impl SymbolTable {
    fn define(&mut self, name: String) -> u16 {
        let index = self.num_definitions;
        let symbol = Symbol {
            name: name.clone(),
            scope: Scope::Global,
            index,
        };
        self.store.insert(name, symbol);
        self.num_definitions += 1;
        index
    }

    fn resolve(&self, name: &String) -> Option<&Symbol> {
        self.store.get(name)
    }
}

#[derive(Debug, Default)]
struct CompilationScope {
    instructions: Vec<Instruction>,
}

impl CompilationScope {}

#[derive(Debug, Default)]
pub struct Emitter {
    instructions: Vec<Instruction>,
    constants: Vec<Object>,
    symbol_table: SymbolTable,
}

impl Emitter {
    pub fn compile_program(&mut self, prog: Program) {
        let prog = Node::Program(prog);
        self.compile(&prog);
    }

    fn compile(&mut self, root: &Node) {
        match root {
            Node::Program(prog) => {
                for stmt in &prog.statements {
                    self.compile_statement(stmt);
                }
            }
            Node::Statement(stmt) => {
                self.compile_statement(stmt);
            }
            Node::Expression(expr) => {
                self.compile_expression(expr);
                // TODO
                self.instructions.push(Instruction::Pop);
            }
        }
    }

    fn compile_statement(&mut self, stmt: &StatementNode) {
        match stmt {
            StatementNode::Let(_, ident, expr) => {
                self.compile_expression(expr);
                let index = self.symbol_table.define(ident.get_string());
                self.instructions.push(Instruction::SetGlobal(index));
            }
            StatementNode::Expression(expr) => {
                self.compile_expression(expr);
                self.instructions.push(Instruction::Pop);
            }
            _ => todo!(),
        }
    }

    fn compile_expression(&mut self, expr: &ExpressionNode) {
        match expr {
            ExpressionNode::Ident(token) => {
                let name = token.clone().get_ident_name().unwrap();
                if let Some(sym) = self.symbol_table.resolve(&name) {
                    self.instructions.push(Instruction::GetGlobal(sym.index));
                }
            }
            ExpressionNode::Int(token) => {
                if let TokenType::Int(ref int) = token.r#type {
                    let int = int.parse::<isize>().unwrap();
                    let i = Object::Int(int);
                    self.constants.push(i);
                    self.instructions
                        .push(Instruction::Constant(self.constants.len() as u16 - 1))
                }
            }
            ExpressionNode::Bool(token) => {
                let o = match token.r#type {
                    TokenType::True => Object::Bool(true),
                    TokenType::False => Object::Bool(false),
                    _ => unreachable!(),
                };
                self.constants.push(o);
                self.instructions
                    .push(Instruction::Constant(self.constants.len() as u16 - 1))
            }
            ExpressionNode::UnaryOperator(operator, operand) => {
                self.compile_expression(operand);
                let operator = match operator.r#type {
                    TokenType::Bang => Instruction::Not,
                    TokenType::Minus => Instruction::UnaryMinus,
                    TokenType::Plus => Instruction::UnaryPlus,
                    _ => unreachable!(),
                };
                self.instructions.push(operator);
            }
            ExpressionNode::BinaryOperator(operator, left, right) => {
                self.compile_expression(left);
                self.compile_expression(right);
                let operator = match operator.r#type {
                    TokenType::Plus => Instruction::Add,
                    TokenType::Minus => Instruction::Sub,
                    TokenType::Asterisk => Instruction::Mul,
                    TokenType::Slash => Instruction::Div,
                    TokenType::Eq => Instruction::Eq,
                    TokenType::NotEq => Instruction::NotEq,
                    TokenType::GT => Instruction::GT,
                    TokenType::LT => Instruction::LT,
                    _ => unreachable!(),
                };
                self.instructions.push(operator);
            }
            ExpressionNode::Block(stmts) => {
                for stmt in stmts {
                    self.compile_statement(stmt);
                }
                if matches!(
                    self.instructions[self.instructions.len() - 1],
                    Instruction::Pop
                ) {
                    self.instructions.pop();
                }
            }
            ExpressionNode::If(_, cond, action, alternate) => {
                self.compile_expression(cond);
                self.instructions.push(Instruction::JumpNotTruthy(0));
                let ai = self.instructions.len();
                self.compile_expression(&ExpressionNode::Block(action.statements.clone()));
                {
                    let a_act = self.instructions.len();
                    self.instructions[ai - 1] = Instruction::JumpNotTruthy(a_act - 1);
                }
                if let Some(alternate) = alternate {
                    self.instructions.push(Instruction::Jump(0));
                    let a_act = self.instructions.len();
                    {
                        let a_act = self.instructions.len();
                        self.instructions[ai - 1] = Instruction::JumpNotTruthy(a_act - 1);
                    }
                    self.compile_expression(&ExpressionNode::Block(alternate.statements.clone()));
                    {
                        let a_alt = self.instructions.len();
                        self.instructions[a_act - 1] = Instruction::Jump(a_alt - 1);
                    }
                }
            }
            _ => todo!(),
        }
    }

    pub fn emit(&self) -> Bytecode {
        let mut instructions: Vec<_> = self.instructions.iter().map(|i| i.to_bytes()).collect();
        let lengths: Vec<_> = instructions
            .iter()
            .map(|v| v.len())
            .scan(0, |acc, x| {
                *acc += x;
                Some(*acc)
            })
            .collect();
        for (i, inst) in self.instructions.iter().enumerate() {
            let index = match inst {
                Instruction::Jump(index) | Instruction::JumpNotTruthy(index) => lengths[*index],
                _ => continue,
            };
            instructions[i] =
                instructions[i][..instructions[i].len() - std::mem::size_of::<usize>()].to_vec();
            instructions[i].extend(index.to_be_bytes());
        }
        Bytecode {
            instructions: instructions.into_iter().flatten().collect(),
            constants: self.constants.clone(),
        }
    }
}
