use std::fmt::Display;

use derive_more::Constructor;

use crate::lexer::{Pos, Token, TokenValue};

#[derive(Debug, Constructor)]
pub struct Error {
    pub kind: ErrorKind,
    pub pos: Pos,
}

#[derive(Debug)]
pub enum ErrorKind {
    UnexpectedChar(char),
    UnexpectedToken(TokenValue),
    ReturnOutsideFunction,
    FunctionInBlock,
}

impl Error {
    pub fn unexpected_token(Token { value, pos }: Token) -> Self {
        let kind = ErrorKind::UnexpectedToken(value);

        Error::new(kind, pos)
    }

    pub fn return_outside_function(pos: Pos) -> Self {
        let kind = ErrorKind::ReturnOutsideFunction;

        Error::new(kind, pos)
    }

    pub fn function_in_block(pos: Pos) -> Self {
        let kind = ErrorKind::FunctionInBlock;

        Error::new(kind, pos)
    }
}

impl Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorKind::UnexpectedChar(char) => write!(f, "unexpected char '{char}'"),
            ErrorKind::UnexpectedToken(token) => write!(f, "unexpected token \"{token}\""),
            ErrorKind::ReturnOutsideFunction => write!(f, "using return outside the function"),
            ErrorKind::FunctionInBlock => todo!(),
        }
    }
}
