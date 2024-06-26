use std::fmt;

use smplc_ast::Span;
use smplc_lexer::{Token, TokenValue};

pub type ParseResult<'source, T> = Result<T, ParseError<'source>>;

#[derive(Debug)]
pub struct ParseError<'source> {
    pub kind: ParseErrorKind<'source>,
    pub span: Span,
}

#[derive(Debug)]
pub enum ParseErrorKind<'source> {
    UnexpectedToken(TokenValue<'source>),
    BreakOutsideLoop,
    ContinueOutsideLoop,
}

impl<'source> ParseError<'source> {
    pub fn unexpected_token(Token { value, span }: Token<'source>) -> Self {
        let kind = ParseErrorKind::UnexpectedToken(value);

        Self { kind, span }
    }
}

impl fmt::Display for ParseErrorKind<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnexpectedToken(token) => {
                write!(f, "unexpected token \"{token}\"")
            }

            Self::BreakOutsideLoop => {
                write!(f, "using break outside loop")
            }

            Self::ContinueOutsideLoop => {
                write!(f, "using continue outside loop")
            }
        }
    }
}
