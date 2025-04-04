use std::fmt;

use smplc_ast::Span;
use smplc_lexer::{Token, TokenTag};

pub type ParseResult<'source, T> = Result<T, ParseError<'source>>;

#[derive(Debug)]
pub struct ParseError<'source> {
    pub kind: ParseErrorKind<'source>,
    pub span: Span,
}

#[derive(Debug)]
pub enum ParseErrorKind<'source> {
    UnexpectedToken(TokenTag, &'source str),
    UnexpectedChar(char),
    InvalidSwizzle,
    BreakOutsideLoop,
    ContinueOutsideLoop,
}

impl<'source> ParseError<'source> {
    pub fn unexpected_token(Token { tag, value, span }: Token<'source>) -> Self {
        let kind = ParseErrorKind::UnexpectedToken(tag, value);

        Self { kind, span }
    }

    pub fn invalid_swizzle(span: Span) -> Self {
        Self {
            kind: ParseErrorKind::InvalidSwizzle,
            span,
        }
    }
}

impl fmt::Display for ParseErrorKind<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnexpectedToken(_, value) => {
                write!(f, "unexpected token \"{value}\"")
            }

            Self::BreakOutsideLoop => {
                write!(f, "using break outside loop")
            }

            Self::InvalidSwizzle => {
                write!(f, "invalid swizzle combination")
            }

            Self::ContinueOutsideLoop => {
                write!(f, "using continue outside loop")
            }

            Self::UnexpectedChar(char) => {
                write!(f, "unexpected char '{char}'")
            }
        }
    }
}

impl From<smplc_lexer::LexError> for ParseError<'_> {
    fn from(value: smplc_lexer::LexError) -> Self {
        Self {
            kind: ParseErrorKind::UnexpectedChar(value.char),
            span: Span::with_len(value.pos, 1),
        }
    }
}
