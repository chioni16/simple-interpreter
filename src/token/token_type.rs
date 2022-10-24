#[derive(Debug, Clone)]
pub enum TokenType {
    Illegal,
    Eof,
    // Identifiers + literals
    Ident(String), 
    Int(String), // remains a string as I don't want to "parse" the data till the parse step
    // Operators
    Assign, 
    Plus,
    Minus, 
    Asterisk, 
    Slash, 
    LT,
    GT,
    Bang,
    // Delimiters
    Comma, 
    Semicolon, 
    Lparen,
    Rparen,
    Lbrace,
    Rbrace,
    // Keywords
    Function, 
    Let, 
    True,
    False,
    If,
    Else,
    Return,
}

pub(crate) fn token_type_for_special_symbols(c: char) -> Option<TokenType> {
    let tt = match c {
        '=' => TokenType::Assign,
        '+' => TokenType::Plus,
        '-' => TokenType::Minus,
        '*' => TokenType::Asterisk,
        '/' => TokenType::Slash,
        '<' => TokenType::LT,
        '>' => TokenType::GT,
        '!' => TokenType::Bang,

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

const KEYWORDS: [(&'static str, TokenType); 7] = [
    ("fn", TokenType::Function),
    ("let", TokenType::Let),
    ("true", TokenType::True), 
    ("false", TokenType::False),
    ("if", TokenType::If),
    ("else", TokenType::Else),
    ("return", TokenType::Return),
];

pub(crate) fn is_keyword<'a>(s: impl Into<&'a str>) -> Option<&'static TokenType> {
    let s = s.into();
    KEYWORDS.iter()
        .find(|e| s.eq((**e).0))
        .map(|(_, tt)| tt)
} 