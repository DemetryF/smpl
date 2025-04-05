use smplc_ast::{MakeSpanned, Pos, Span, Spanned};
use smplc_lexer::{LexError, Token, TokenTag};

use crate::error::{ParseError, ParseResult};

pub trait Tokens<'source>: Iterator<Item = Result<Token<'source>, LexError>> {}
impl<'source, T: Iterator<Item = Result<Token<'source>, LexError>>> Tokens<'source> for T {}

pub struct TokenStream<'source, TS: Tokens<'source>> {
    tokens: TS,
    current: Token<'source>,

    prev_span: Span,

    pub in_loop: bool,
}

impl<'source, TS: Tokens<'source>> TokenStream<'source, TS> {
    pub fn new(mut tokens: TS) -> Result<Self, LexError> {
        Ok(Self {
            current: tokens.next().unwrap()?,
            tokens,
            prev_span: Span::default(),
            in_loop: false,
        })
    }

    pub fn current(&self) -> Token<'source> {
        self.current
    }

    pub fn check(&self, value: TokenTag) -> bool {
        !self.is_end() && self.current().tag == value
    }

    pub fn consume(&mut self, value: TokenTag) -> ParseResult<'source, Token> {
        if self.check(value) {
            Ok(self.next_token()?)
        } else {
            Err(self.unexpected_token())
        }
    }

    pub fn try_consume(&mut self, value: TokenTag) -> ParseResult<'source, bool> {
        if self.check(value) {
            self.next_token()?;

            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub fn next_token(&mut self) -> ParseResult<'source, Token<'source>> {
        let token = self.current();

        self.prev_span = token.span;

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
        self.current().tag == TokenTag::EOF
    }

    pub fn work<T>(
        &mut self,
        mut f: impl FnMut(&mut Self) -> ParseResult<'source, T>,
    ) -> ParseResult<'source, Spanned<T>> {
        let start = self.current.span;
        let value = f(self);
        let end = self.prev_span;

        value.map(|value| value.spanned(Span::unite(start, end)))
    }
}
