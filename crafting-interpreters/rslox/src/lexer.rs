/*!
 * Module for lexer/scanner
 */

use self::Token::*;
use crate::token::Token;
use std::string::String;

pub(crate) struct Lexer {
    source: String,
    tokens: Vec<EnrichedToken>,
    start: i32,
    current: i32,
    line: i32,
}

impl Lexer {
    pub fn new(source: &str) -> Self {
        Self {
            source: source.to_string(),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        while !is_at_end() {
            self.start = self.current;
            scan_token();
        }

        self.tokens
            .push(EnrichedToken::new(EOF, "".to_string(), line));
        self.tokens
    }
}

/// Token types
enum Token {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,
    Bang,
    BangEq,
    Equal,
    EqualEqual,
    GreaterThan,
    GreaterEqual,
    LessThan,
    LessEqual,
    Identifier,
    String,
    Number,
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
    EOF,
}

struct EnrichedToken {
    token: Token,
    text: String,
    line: i32,
}

impl EnrichedToken {
    fn new(token: Token, text: String, line: i32) -> Self {
        EnrichedToken { token, text, line }
    }
}
