use crate::token::{ 
    token_type::{TokenType, tt_keywords, tt_single_operators, tt_double_operators, tt_delimiters}, 
    Token,
};

#[derive(Debug)]
pub struct Lexer {
    // input: BufReader<T>,
    // keeping it simple for now
    // you can use BufReader with the unicode adapter that I have made 
    // currently supports only pure ascii input
    input: Vec<u8>, 

    // state 
    // if you end up using bufreader, the state is managed by it? 
    // not sure of the api that it exposes and the guarantees provided

    // indices into input
    // correspond to the lexeme/token we are currently looking at
    // 0 <= start_pos <= end_pos < len(input)
    start_pos: usize,
    end_pos: usize,
}

impl Lexer {
    pub fn from_string(input: String) -> Self {
        let mut input = input.as_bytes().to_vec();
        // for making it simpler for pattern matching
        input.push(0);
        input.push(0);

        Self {
            input,
            start_pos: 0,
            end_pos: 0,
        }
    }
    // maybe another fn from_file? 
}

// methods for managing state
// i.e, cruising through the input
impl Lexer {
    fn step_one(&mut self) {
        if self.end_pos >= self.input.len() {
            return;
        }
        self.end_pos += 1;
    }

    fn step_until(&mut self, f: fn(u8) -> bool) -> Vec<u8> {
        // first condition is required to prevent infinite loops
        // for inputs that end with whitespace
        while self.end_pos < self.input.len() && f(self.input[self.end_pos]) {
            self.step_one();
        }
        let lexeme = &self.input[self.start_pos..self.end_pos];
        lexeme.to_owned()
    }

    fn skip_ws(&mut self) {
        // I don't plan to use whitespace information currently for my target language
        // But if you ever need it you can grasp it from here, i guess
        self.step_until(|c| c.is_ascii_whitespace());
    }

    fn ident(&mut self) -> String {
        let v = self.step_until(|c| c.is_ascii_alphanumeric());
        // safe to unwrap as they are all ascii alphanumeric chars
        String::from_utf8(v).unwrap()
    }

    fn digits(&mut self) -> String {
        let v = self.step_until(|c| c.is_ascii_digit());
        // safe to unwrap as they are all ascii alphanumeric chars
        String::from_utf8(v).unwrap()
    }
}

impl Lexer {
    fn create_token(&self, tt: TokenType) -> Token {
        Token::new(tt, (self.start_pos, self.end_pos))
    }

    fn next_token(&mut self) -> Token {
        self.skip_ws();
        self.start_pos = self.end_pos;
        // invariance: start_pos is pointing to the beginning of a new lexeme
        // when we reach the match statement
        let c0 = self.input[self.start_pos] as char;
        let c1 = self.input[self.start_pos+1] as char;
        match (c0, c1) {
            ('\x00', _) => self.create_token(TokenType::Eof),
            ('a'..='z' | 'A'..='Z', _) => {
                let s = self.ident();
                if let Some(kw) = tt_keywords(s.as_str()) {
                    self.create_token(kw)
                } else {
                    self.create_token(TokenType::Ident(s))
                }
            }
            ('0'..='9', _) => {
                let s = self.digits();
                self.create_token(TokenType::Int(s))
            }
            (c0, c1) => {
                if let Some(tt) = tt_double_operators(c0, c1) {
                    self.end_pos += 2;
                    self.create_token(tt)
                } else if let Some(tt) = tt_single_operators(c0).or_else(|| tt_delimiters(c0)) {
                    self.end_pos += 1;
                    self.create_token(tt)
                } else {
                    self.create_token(TokenType::Illegal)
                }
            }
        }
    }
}
// Exposes the iterator interface
// Some other options :
//      do what rob pike did and use channels? 
//      https://www.youtube.com/watch?v=HxaD_trXwRE
impl Iterator for Lexer {
    type Item = Token;
    fn next(&mut self) -> Option<Self::Item> {
        let token = self.next_token();
        match token.r#type {
            TokenType::Eof => None,
            _ => Some(token),
        }
    }
}