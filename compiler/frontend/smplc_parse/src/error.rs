use std::fmt::Display;

use derive_more::Constructor;

use smplc_ast::Pos;
use smplc_lexer::{Token, TokenValue};

pub type ParseResult<'source, T> = Result<T, ParseError<'source>>;

#[derive(Debug, Constructor)]
pub struct ParseError<'source> {
    pub kind: ParseErrorKind<'source>,
    pub pos: Pos,
}

#[derive(Debug)]
pub enum ParseErrorKind<'source> {
    UnexpectedToken(TokenValue<'source>),
    ReturnOutsideFunction,
    FunctionInBlock,
}

impl<'source> ParseError<'source> {
    pub fn unexpected_token(Token { value, pos }: Token<'source>) -> Self {
        let kind = ParseErrorKind::UnexpectedToken(value);

        ParseError::new(kind, pos)
    }

    pub fn return_outside_function(pos: Pos) -> Self {
        let kind = ParseErrorKind::ReturnOutsideFunction;

        ParseError::new(kind, pos)
    }

    pub fn function_in_block(pos: Pos) -> Self {
        let kind = ParseErrorKind::FunctionInBlock;

        ParseError::new(kind, pos)
    }
}

impl Display for ParseErrorKind<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseErrorKind::UnexpectedToken(token) => {
                write!(f, "unexpected token \"{token}\"")
            }

            ParseErrorKind::ReturnOutsideFunction => {
                write!(f, "using return outside the function")
            }

            ParseErrorKind::FunctionInBlock => {
                write!(f, "functions are not allowed in blocks")
            }
        }
    }
}
