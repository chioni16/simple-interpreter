use std::mem::discriminant;

use crate::ast::{
    Program,
    expression::{
        BinaryOperator, Block, Bool, ExpressionNode, Function, FunctionCall, Ident, If, Int,
        UnaryOperator,
    },
    statement::{ExpressionStatement, LetStatement, ReturnStatement, StatementNode},
};
use crate::lexer::Lexer;
use crate::token::{token_type::TokenType, Token};

#[allow(dead_code)]
#[derive(Debug)]
pub struct ParseError {
    expected: String,
    found: Option<Token>,
}

type ParseResult<T> = Result<T, ParseError>;

pub struct Parser {
    lexer: Lexer,
    current: Option<Token>,
    peek: Option<Token>,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        let mut p = Self {
            lexer,
            current: None,
            peek: None,
        };
        p.advance_tokens();
        p.advance_tokens();
        p
    }

    fn advance_tokens(&mut self) {
        // self.current = self.peek.replace(self.lexer.next());
        // TODO what happens if i keep calling next on lexer
        // even after it returned None?
        let next = self.lexer.next();
        self.current = self.peek.take();
        self.peek = next;
    }

    fn check_token_type(&self, expected: TokenType) -> ParseResult<()> {
        match self.current.as_ref() {
            Some(Token { r#type: tt, .. }) if discriminant(&expected) == discriminant(tt) => Ok(()),
            _ => Err(ParseError {
                expected: format!("{:?}", expected),
                found: self.current.clone(),
            }),
        }
    }

    fn expect(&mut self, expected: TokenType) -> ParseResult<()> {
        self.check_token_type(expected)?;
        self.advance_tokens();
        Ok(())
    }
}

// invariance: this is followed by all the parse functions(aka parselets) in this project
// a little bit like the target arch calling conventions
// when a parselet is called, the `current` token points to the token that belongs to the AST node returned by the parselet
impl Parser {
    pub fn parse_program(&mut self) -> ParseResult<Program> {
        let mut program = Program::new();
        while self.current.is_some() {
            let stmt = self.parse_statement()?;
            program.add_statement(stmt);
        }
        Ok(program)
    }

    fn parse_statement(&mut self) -> ParseResult<StatementNode> {
        // safe to unwrap because of the check in parse_program function
        let stmt: StatementNode = match self.current.as_ref().unwrap().r#type {
            TokenType::Let => self.parse_let_statement()?.into(),
            TokenType::Return => self.parse_return_statement()?.into(),
            _ => self.parse_expression_statement()?.into(),
        };
        self.expect(TokenType::Semicolon).ok();
        Ok(stmt)
    }

    fn parse_let_statement(&mut self) -> ParseResult<LetStatement> {
        self.expect(TokenType::Let)?;

        let ident = self.parse_ident()?;

        self.expect(TokenType::Assign)?;

        let expr = self.parse_expression(0)?;

        let stmt = LetStatement::new(ident, expr);
        Ok(stmt)
    }

    fn parse_return_statement(&mut self) -> ParseResult<ReturnStatement> {
        self.expect(TokenType::Return)?;

        let expr = self.parse_expression(0)?;

        let stmt = ReturnStatement::new(expr);
        Ok(stmt)
    }

    fn parse_expression_statement(&mut self) -> ParseResult<ExpressionStatement> {
        let expr = self.parse_expression(0)?;

        let stmt = ExpressionStatement::new(expr);

        Ok(stmt)
    }
}
impl Parser {
    fn parse_ident(&mut self) -> ParseResult<Ident> {
        self.check_token_type(TokenType::Ident("".into()))?;
        let ident = Ident::new(self.current.take().unwrap());
        self.advance_tokens();
        Ok(ident)
    }

    fn parse_int(&mut self) -> ParseResult<Int> {
        self.check_token_type(TokenType::Int("".into()))?;
        let int = Int::new(self.current.take().unwrap());
        self.advance_tokens();
        Ok(int)
    }

    fn parse_bool(&mut self) -> ParseResult<Bool> {
        // no need for the check here i suppose
        let b = Bool::new(self.current.take().unwrap());
        self.advance_tokens();
        Ok(b)
    }

    fn parse_block(&mut self) -> ParseResult<Block> {
        self.expect(TokenType::Lbrace)?;

        let mut stmts = vec![];
        while self.check_token_type(TokenType::Rbrace).is_err() {
            stmts.push(self.parse_statement()?);
        }

        self.expect(TokenType::Rbrace)?;

        Ok(Block::new(stmts))
    }

    fn parse_if_else(&mut self) -> ParseResult<If> {
        self.expect(TokenType::If)?;

        let condition = self.parse_expression(0)?;

        let action = self.parse_block()?;

        let mut alternate = None;
        if self.expect(TokenType::Else).is_ok() {
            alternate = Some(self.parse_block()?);
        }

        Ok(If::new(condition, action, alternate))
    }

    fn parse_arg_list(&mut self) -> ParseResult<Vec<Ident>> {
        self.expect(TokenType::Lparen)?;
        let mut args = vec![];
        let mut tc = Ok(());
        while self.check_token_type(TokenType::Rparen).is_err() {
            tc?;
            args.push(self.parse_ident()?);
            tc = self.expect(TokenType::Comma);
        }
        tc.ok();
        self.expect(TokenType::Rparen)?;
        Ok(args)
    }

    fn parse_call_arg_list(&mut self) -> ParseResult<Vec<ExpressionNode>> {
        self.expect(TokenType::Lparen)?;
        let mut args = vec![];
        let mut tc = Ok(());
        while self.check_token_type(TokenType::Rparen).is_err() {
            tc?;
            args.push(self.parse_expression(0)?);
            tc = self.expect(TokenType::Comma);
        }
        tc.ok();
        self.expect(TokenType::Rparen)?;
        Ok(args)
    }

    fn parse_function(&mut self) -> ParseResult<Function> {
        self.expect(TokenType::Function)?;

        let args = self.parse_arg_list()?;

        let body = self.parse_block()?;

        Ok(Function::new(args, body))
    }

    fn parse_expression(&mut self, prec: i8) -> ParseResult<ExpressionNode> {
        if self.current.is_none() {
            Err(ParseError {
                expected: "Any Expression".into(),
                found: self.current.take(),
            })?;
        }
        let mut left: ExpressionNode = match self.current.as_ref().unwrap().r#type {
            TokenType::Ident(_) => self.parse_ident()?.into(),
            TokenType::Int(_) => self.parse_int()?.into(),
            TokenType::True | TokenType::False => self.parse_bool()?.into(),
            TokenType::Lparen => {
                self.advance_tokens();
                let expr = self.parse_expression(0)?;
                self.expect(TokenType::Rparen)?;
                expr
            }
            TokenType::Plus | TokenType::Minus | TokenType::Bang => {
                let operator = self.current.take().unwrap();
                self.advance_tokens();
                let operand = self.parse_expression(100)?.into();
                UnaryOperator::new(operator, operand).into()
            }
            TokenType::Lbrace => self.parse_block()?.into(),
            TokenType::If => self.parse_if_else()?.into(),
            TokenType::Function => self.parse_function()?.into(),
            // TokenType::Semicolon => return Ok(ExpressionNode::None),
            _ => Err(ParseError {
                expected: "Ident|Int|UnaryOperator|(|{|if|fn".into(),
                found: self.current.take(),
            })?,
        };
        let mut nop = get_prec_assoc(self.current.as_ref());
        while prec <= nop {
            if let Some(Token {
                r#type: TokenType::Lparen,
                ..
            }) = self.current
            {
                let args = self.parse_call_arg_list()?;
                left = FunctionCall::new(left, args).into();
            } else {
                let bop = self.current.take().unwrap();
                self.advance_tokens();
                let right = self.parse_expression(nop)?;
                left = BinaryOperator::new(bop, left, right).into();
            }

            nop = get_prec_assoc(self.current.as_ref());
        }
        // self.expect(TokenType::Semicolon).ok();
        Ok(left)
    }
}

fn get_prec_assoc(op: Option<&Token>) -> i8 {
    // Precedence + Associativity (left=0;right=5)
    match op {
        Some(Token {
            r#type: TokenType::Plus,
            ..
        }) => 30 + 0,
        Some(Token {
            r#type: TokenType::Minus,
            ..
        }) => 30 + 0,
        Some(Token {
            r#type: TokenType::Asterisk,
            ..
        }) => 40 + 0,
        Some(Token {
            r#type: TokenType::Slash,
            ..
        }) => 40 + 0,

        Some(Token {
            r#type: TokenType::Eq,
            ..
        }) => 20 + 0,
        Some(Token {
            r#type: TokenType::NotEq,
            ..
        }) => 20 + 0,
        Some(Token {
            r#type: TokenType::GT,
            ..
        }) => 20 + 0,
        Some(Token {
            r#type: TokenType::LT,
            ..
        }) => 20 + 0,
        Some(Token {
            r#type: TokenType::Lparen,
            ..
        }) => 110 + 0,

        Some(Token {
            r#type: TokenType::Rparen,
            ..
        }) => -1 + 0,
        _ => -100 + 0,
    }
}
