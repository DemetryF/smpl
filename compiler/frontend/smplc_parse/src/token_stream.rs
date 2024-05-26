use smplc_ast::{Pos, Span};
use smplc_lexer::{Token, TokenValue};

use crate::error::{ParseError, ParseResult};

#[derive(Default)]
pub struct TokenStream<'source> {
    tokens: Vec<Token<'source>>,
    index: usize,

    pub in_loop: bool,
}

impl<'source> TokenStream<'source> {
    pub fn new(tokens: Vec<Token<'source>>) -> Self {
        Self {
            tokens,
            ..Default::default()
        }
    }

    pub fn current(&self) -> Token<'source> {
        self.tokens[self.index]
    }

    pub fn check(&self, value: TokenValue) -> bool {
        !self.is_end() && self.current().value == value
    }

    pub fn consume(&mut self, value: TokenValue) -> ParseResult<'source, Span> {
        if self.check(value) {
            let span = self.next_token().span;

            return Ok(span);
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

    pub fn next_token(&mut self) -> Token<'source> {
        let token = self.current();

        self.index += 1;

        token
    }

    pub fn unexpected_token(&self) -> ParseError<'source> {
        ParseError::unexpected_token(self.current())
    }

    pub fn get_pos(&self) -> Pos {
        self.current().span.start()
    }

    pub fn is_end(&self) -> bool {
        self.current().value == TokenValue::EOF
    }
}
