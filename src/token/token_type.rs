#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenType {
    Illegal,
    Eof,

    Ident(String), 
    Int(String), // remains a string as I don't want to "parse" the data till the parse step

    SO(SingleOperator),
    DO(DoubleOperator),
    DL(Delimiter),
    KW(Keyword),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SingleOperator {
    // Operators
    Assign, 
    Plus,
    Minus, 
    Asterisk, 
    Slash, 
    LT,
    GT,
    Bang,
}

pub(crate) fn tt_single_operators(c: char) -> Option<TokenType> {
    let tt = match c {
        '=' => SingleOperator::Assign,
        '+' => SingleOperator::Plus,
        '-' => SingleOperator::Minus,
        '*' => SingleOperator::Asterisk,
        '/' => SingleOperator::Slash,
        '<' => SingleOperator::LT,
        '>' => SingleOperator::GT,
        '!' => SingleOperator::Bang,
        _ => return None,
    };
    Some(TokenType::SO(tt))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DoubleOperator {
    Eq,
    NotEq,
}

pub(crate) fn tt_double_operators(c0: char, c1: char) -> Option<TokenType> {
    let tt = match (c0, c1) {
        ('=', '=') => DoubleOperator::Eq,
        ('!', '=') => DoubleOperator::NotEq,
        _ => return None
    };
    Some(TokenType::DO(tt))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Delimiter {
    Comma, 
    Semicolon, 
    Lparen,
    Rparen,
    Lbrace,
    Rbrace,
}

pub(crate) fn tt_delimiters(c: char) -> Option<TokenType> {
    let tt = match c {
        ',' => Delimiter::Comma,
        ';' => Delimiter::Semicolon,
        '(' => Delimiter::Lparen,
        ')' => Delimiter::Rparen,
        '{' => Delimiter::Lbrace,
        '}' => Delimiter::Rbrace,
        _ => return None,
    };
    Some(TokenType::DL(tt))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Keyword {
    Function, 
    Let, 
    True,
    False,
    If,
    Else,
    Return,
}

pub(crate) fn tt_keywords<'a>(s: impl Into<&'a str>) -> Option<TokenType> {
    let tt = match s.into() {
        "fn" => Keyword::Function,
        "let" => Keyword::Let,
        "true" => Keyword::True, 
        "false" => Keyword::False,
        "if" => Keyword::If,
        "else" => Keyword::Else,
        "return" => Keyword::Return,
        _ => return None
    };
    Some(TokenType::KW(tt))
} 
