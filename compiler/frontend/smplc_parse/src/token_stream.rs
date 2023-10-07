use std::{iter::Peekable, vec::IntoIter};

use smplc_lexer::token::{Token, TokenValue};

use crate::error::{ParseError, ParseResult};

pub struct TokenStream<'source> {
    tokens: Peekable<IntoIter<Token<'source>>>,

    pub in_function: bool,
    pub in_cycle: bool,
}

impl<'source> TokenStream<'source> {
    pub fn new(tokens: IntoIter<Token<'source>>) -> Self {
        Self {
            tokens: tokens.peekable(),

            in_function: false,
            in_cycle: false,
        }
    }

    pub fn current(&mut self) -> &Token<'source> {
        self.tokens.peek().unwrap()
    }

    pub fn check(&mut self, value: TokenValue) -> bool {
        self.current().value == value
    }

    pub fn next(&mut self) -> Token<'source> {
        self.tokens.next().unwrap()
    }

    pub fn consume(&mut self, value: TokenValue) -> ParseResult<'source, Token<'source>> {
        if self.check(value) {
            Ok(self.next())
        } else {
            Err(self.unexpected_token())
        }
    }

    pub fn try_consume(&mut self, value: TokenValue<'source>) -> bool {
        if self.check(value) {
            self.next();

            true
        } else {
            false
        }
    }

    pub fn unexpected_token(&mut self) -> ParseError<'source> {
        ParseError::unexpected_token(self.current().to_owned())
    }

    pub fn is_end(&mut self) -> bool {
        self.check(TokenValue::EOF)
    }
}
