use smplc_ast::{Pos, Span};
use smplc_lexer::{LexError, Token, TokenValue};

use crate::error::{ParseError, ParseResult};

pub trait Tokens<'source>: Iterator<Item = Result<Token<'source>, LexError>> {}
impl<'source, T: Iterator<Item = Result<Token<'source>, LexError>>> Tokens<'source> for T {}

pub struct TokenStream<'source, TS: Tokens<'source>> {
    tokens: TS,
    current: Token<'source>,

    pub in_loop: bool,
}

impl<'source, TS: Tokens<'source>> TokenStream<'source, TS> {
    pub fn new(mut tokens: TS) -> Result<Self, LexError> {
        Ok(Self {
            current: tokens.next().unwrap()?,
            tokens,
            in_loop: false,
        })
    }

    pub fn current(&self) -> Token<'source> {
        self.current
    }

    pub fn check(&self, value: TokenValue) -> bool {
        !self.is_end() && self.current().value == value
    }

    pub fn consume(&mut self, value: TokenValue) -> ParseResult<'source, Span> {
        if self.check(value) {
            let span = self.next_token()?.span;

            return Ok(span);
        }

        Err(self.unexpected_token())
    }

    pub fn try_consume(&mut self, value: TokenValue) -> ParseResult<'source, bool> {
        if self.check(value) {
            self.next_token()?;

            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub fn next_token(&mut self) -> ParseResult<'source, Token<'source>> {
        let token = self.current();

        self.current = self.tokens.next().unwrap()?;

        Ok(token)
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
