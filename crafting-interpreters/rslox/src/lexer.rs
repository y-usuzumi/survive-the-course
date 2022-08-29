use crate::token::Token;

pub struct Lexer {
    source: String,
}

impl Lexer {
    pub fn new(source: &str) -> Self {
        Self {
            source: source.to_string(),
        }
    }
    pub fn tokenize(&self) -> Vec<Token> {
        Vec::new()
    }
}
