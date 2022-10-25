use std::mem::discriminant;

use crate::ast::{StatementNode, ExpressionNode};
use crate::ast::expression::{Ident, Int, UnaryOperator, BinaryOperator, Bool, Block, If, Function, ArgList, FunctionCall};
use crate::token::Token;
use crate::lexer::Lexer;
use crate::ast::statement::{Program, LetStatement, ReturnStatement, ExpressionStatement};
use crate::token::token_type::{ TokenType, Keyword, SingleOperator, Delimiter };

#[derive(Debug)]
pub struct ParseError {
    expected: String, 
    found: Option<Token>, 
}
// impl ParseError {
//     fn new(expected: Vec<TokenType>, found: Token) -> Self {
//         Self {
//             expected, 
//             found,
//         }
//     }
// }
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
        if let Some(t) = self.current.as_ref() {
            let tt = &t.r#type;
            match expected {
                TokenType::Ident(_) | TokenType::Int(_) if discriminant(&expected) == discriminant(tt) => { return Ok(()) } 
                _  => if t.r#type == expected { return Ok(()) },   
            }
        }
        Err(ParseError {
            expected: format!("{:?}", expected),
            found: self.current.clone(),
        })
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

    fn parse_statement(&mut self) -> ParseResult<Box<dyn StatementNode>> {
        // safe to unwrap because of the check in parse_program function
        let stmt: Box<dyn StatementNode> = match self.current.as_ref().unwrap().r#type {
            TokenType::KW(Keyword::Let) => Box::from(self.parse_let_statement()?),
            TokenType::KW(Keyword::Return) => Box::from(self.parse_return_statement()?),
            _ => Box::from(self.parse_expression_statement()?),
        };
        Ok(stmt)
    }

    fn parse_let_statement(&mut self) -> ParseResult<LetStatement> {
        self.expect(TokenType::KW(Keyword::Let))?;

        let ident = self.parse_ident()?;

        self.expect(TokenType::SO(SingleOperator::Assign))?;

        let expr = self.parse_expression(0)?;

        let stmt = LetStatement::new(ident, expr);
        Ok(stmt)
    }

    fn parse_return_statement(&mut self) -> ParseResult<ReturnStatement> {
        self.expect(TokenType::KW(Keyword::Return))?;
        
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
        // if let Some(&Token{ r#type: TokenType::Ident(_), ..}) = self.current.as_ref() {

        // }
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
        self.expect(TokenType::DL(Delimiter::Lbrace))?;

        let mut stmts = vec![];
        while self.check_token_type(TokenType::DL(Delimiter::Rbrace)).is_err() {
            stmts.push(self.parse_statement()?);
        }
        
        self.expect(TokenType::DL(Delimiter::Rbrace))?;
        
        Ok(Block::new(stmts))
    }

    fn parse_if_else(&mut self) -> ParseResult<If> {
        self.expect(TokenType::KW(Keyword::If))?;

        let condition = self.parse_expression(0)?;
        
        let action = self.parse_block()?;
        
        let mut alternate = None;
        if self.expect(TokenType::KW(Keyword::Else)).is_ok() {
            alternate = Some(self.parse_block()?);
        }

        Ok(If::new(condition, action, alternate))
    }

    fn parse_arg_list(&mut self) -> ParseResult<ArgList> {
        self.expect(TokenType::DL(Delimiter::Lparen))?;
        let mut args = vec![];
        let mut tc = Ok(());
        while self.check_token_type(TokenType::DL(Delimiter::Rparen)).is_err() {
            tc?;
            args.push(self.parse_ident()?);
            tc = self.expect(TokenType::DL(Delimiter::Comma));

        }
        tc.ok();
        self.expect(TokenType::DL(Delimiter::Rparen))?;
        Ok(args)
    }

    fn parse_call_arg_list(&mut self) -> ParseResult<Vec<Box<dyn ExpressionNode>>> {
        self.expect(TokenType::DL(Delimiter::Lparen))?;
        let mut args = vec![];
        let mut tc = Ok(());
        while self.check_token_type(TokenType::DL(Delimiter::Rparen)).is_err() {
            tc?;
            args.push(self.parse_expression(0)?);
            tc = self.expect(TokenType::DL(Delimiter::Comma));

        }
        tc.ok();
        self.expect(TokenType::DL(Delimiter::Rparen))?;
        Ok(args)
    }

    fn parse_function(&mut self) -> ParseResult<Function> {
        self.expect(TokenType::KW(Keyword::Function))?;
        
        let args = self.parse_arg_list()?;

        let body = self.parse_block()?;

        Ok(Function::new(args, body))
    }

    fn parse_function_call(&mut self) -> ParseResult<FunctionCall> {
        unimplemented!()
    }

    fn parse_expression(&mut self, prec: i8) -> ParseResult<Box<dyn ExpressionNode>> {
        if self.current.is_none() {
            Err(ParseError{expected: "Any Expression".into(), found: self.current.take()})?;
        }
        let mut left:Box<dyn ExpressionNode> = match self.current.as_ref().unwrap().r#type {
            TokenType::Ident(_) => Box::from(self.parse_ident()?),
            TokenType::Int(_) => Box::from(self.parse_int()?),
            TokenType::KW(Keyword::True) | TokenType::KW(Keyword::False) => Box::from(self.parse_bool()?),
            TokenType::DL(Delimiter::Lparen) => {
                self.advance_tokens();
                let expr = self.parse_expression(0)?;
                self.expect(TokenType::DL(Delimiter::Rparen))?;
                expr
            }
            TokenType::SO(SingleOperator::Plus) | TokenType::SO(SingleOperator::Minus) | TokenType::SO(SingleOperator::Bang) => {
                let operator = self.current.take().unwrap();
                self.advance_tokens();
                let operand = Box::from(self.parse_expression(100)?);
                Box::from(UnaryOperator::new(operator, operand))
            }
            TokenType::DL(Delimiter::Lbrace) => Box::from(self.parse_block()?),
            TokenType::KW(Keyword::If) => Box::from(self.parse_if_else()?),
            TokenType::KW(Keyword::Function) => Box::from(self.parse_function()?),
            _ => Err(ParseError{expected: "Ident|Int|UnaryOperator|(|{|if|fn".into(), found: self.current.take()})?
        };
        let mut nop = get_prec_assoc(self.current.as_ref());
        while prec <= nop{
            if let Some(Token { r#type: TokenType::DL(Delimiter::Lparen), .. }) = self.current {
                let args = self.parse_call_arg_list()?;
                left = Box::from(FunctionCall::new(left, args));
            } else {
                let bop = self.current.take().unwrap();
                self.advance_tokens();
                let right = self.parse_expression(nop)?;
                left = Box::from(BinaryOperator::new(bop, left, right));
            }

            nop = get_prec_assoc(self.current.as_ref());
        }
        self.expect(TokenType::DL(Delimiter::Semicolon)).ok();
        Ok(left)
    }
}

fn get_prec_assoc(op: Option<&Token>) -> i8 {
    // Precedence + Associativity (left=0;right=5)
    match op {
        Some(Token { r#type: TokenType::SO(SingleOperator::Plus), .. }) => 10+0,
        Some(Token { r#type: TokenType::SO(SingleOperator::Minus), .. }) => 10+0,
        Some(Token { r#type: TokenType::SO(SingleOperator::Asterisk), .. }) => 20+0,
        Some(Token { r#type: TokenType::SO(SingleOperator::Slash), .. }) => 20+0,

        Some(Token { r#type: TokenType::DL(Delimiter::Lparen), .. }) => 110+0,

        Some(Token { r#type: TokenType::DL(Delimiter::Rparen), .. }) => -1+0,
        _ => -100+0,
    }
}