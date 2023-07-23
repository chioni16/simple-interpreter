pub(crate) mod token_type;

use token_type::TokenType;

// currently Pos is just the offset in the file
// where the token in question is found
// you can make it more sophisticated by making it a tuple of line number + offset in the line
type Pos = usize;
// position of first character of the token + that of the last one
type Span = (Pos, Pos);

#[derive(Debug, Clone)]
pub struct Token {
    pub r#type: TokenType,
    pub span: Span,
    // you can add the filename later on
    // but for now, all the programs in the target language will into a single file
}

impl Token {
    pub fn new(r#type: TokenType, span: Span) -> Self {
        Self { r#type, span }
    }
}

impl Default for Token {
    fn default() -> Self {
        Token {
            r#type: TokenType::Dummy,
            span: (0, 0),
        }
    }
}

impl Token {
    pub fn get_ident_name(self) -> Option<String> {
        if let TokenType::Ident(ident) = self.r#type {
            Some(ident)
        } else {
            None
        }
    }
}
