use std::fmt::Display;

use crate::lexer::{Pos, Token};
pub type Result<T> = std::result::Result<T, Error>;

pub enum Error {
    UnexpectedChar { value: char, pos: Pos },
    UnexpectedToken(Token),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnexpectedToken(Token { value, pos }) => {
                write!(f, "unexpected token '{value}' at {pos}")
            }
            Self::UnexpectedChar { value, pos } => {
                write!(f, "unexpected char '{value}' at '{pos}'")
            }
        }
    }
}
