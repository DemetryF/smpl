use std::fmt::Display;

use derive_more::Constructor;

use smplc_lexer::{Pos, Token, TokenValue};

#[derive(Debug, Constructor)]
pub struct ParseError {
    pub kind: ParseErrorKind,
    pub pos: Pos,
}

#[derive(Debug)]
pub enum ParseErrorKind {
    UnexpectedToken(TokenValue),
    ReturnOutsideFunction,
    FunctionInBlock,
}

impl ParseError {
    pub fn unexpected_token(Token { value, pos }: Token) -> Self {
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

impl Display for ParseErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseErrorKind::UnexpectedToken(token) => write!(f, "unexpected token \"{token}\""),
            ParseErrorKind::ReturnOutsideFunction => write!(f, "using return outside the function"),
            ParseErrorKind::FunctionInBlock => write!(f, "functions are not allowed in blocks"),
        }
    }
}
