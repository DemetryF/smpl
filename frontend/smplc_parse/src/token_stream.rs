use smplc_ast::Pos;
use smplc_lexer::{Token, TokenValue};

use crate::error::ParseError;

pub struct TokenStream {
    tokens: Vec<Token>,
    index: usize,

    pub in_function: bool,
}

impl TokenStream {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            index: 0,
            in_function: false,
        }
    }

    pub fn current(&self) -> &Token {
        &self.tokens[self.index]
    }

    pub fn check(&self, value: TokenValue) -> bool {
        !self.is_end() && self.current().value == value
    }

    pub fn consume(&mut self, value: TokenValue) -> Result<(), ParseError> {
        if self.check(value) {
            self.next();

            return Ok(());
        }

        Err(self.unexpected_token())
    }

    pub fn try_consume(&mut self, value: TokenValue) -> bool {
        if self.check(value) {
            self.index += 1;

            return true;
        }

        false
    }

    pub fn next(&mut self) -> Token {
        let token = self.current().clone();

        self.index += 1;

        token
    }

    pub fn unexpected_token(&self) -> ParseError {
        ParseError::unexpected_token(self.current().clone())
    }

    pub fn get_pos(&self) -> Pos {
        self.current().pos
    }

    pub fn is_end(&self) -> bool {
        self.current().value == TokenValue::EOF
    }
}
