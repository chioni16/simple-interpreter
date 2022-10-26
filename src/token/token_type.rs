#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenType {
    Illegal(char),
    Eof,

    Ident(String),
    Int(String), // remains a string as I don't want to "parse" the data till the parse step

    // SingleOperator
    Assign,
    Plus,
    Minus,
    Asterisk,
    Slash,
    LT,
    GT,
    Bang,

    // DoubleOperator
    Eq,
    NotEq,

    // Delimiter
    Comma,
    Semicolon,
    Lparen,
    Rparen,
    Lbrace,
    Rbrace,

    // Keyword
    Function,
    Let,
    True,
    False,
    If,
    Else,
    Return,
}

pub(crate) fn tt_single_operators(c: char) -> Option<TokenType> {
    let tt = match c {
        '=' => TokenType::Assign,
        '+' => TokenType::Plus,
        '-' => TokenType::Minus,
        '*' => TokenType::Asterisk,
        '/' => TokenType::Slash,
        '<' => TokenType::LT,
        '>' => TokenType::GT,
        '!' => TokenType::Bang,
        _ => return None,
    };
    Some(tt)
}

pub(crate) fn tt_double_operators(c0: char, c1: char) -> Option<TokenType> {
    let tt = match (c0, c1) {
        ('=', '=') => TokenType::Eq,
        ('!', '=') => TokenType::NotEq,
        _ => return None,
    };
    Some(tt)
}

pub(crate) fn tt_delimiters(c: char) -> Option<TokenType> {
    let tt = match c {
        ',' => TokenType::Comma,
        ';' => TokenType::Semicolon,
        '(' => TokenType::Lparen,
        ')' => TokenType::Rparen,
        '{' => TokenType::Lbrace,
        '}' => TokenType::Rbrace,
        _ => return None,
    };
    Some(tt)
}

pub(crate) fn tt_keywords<'a>(s: impl Into<&'a str>) -> Option<TokenType> {
    let tt = match s.into() {
        "fn" => TokenType::Function,
        "let" => TokenType::Let,
        "true" => TokenType::True,
        "false" => TokenType::False,
        "if" => TokenType::If,
        "else" => TokenType::Else,
        "return" => TokenType::Return,
        _ => return None,
    };
    Some(tt)
}
