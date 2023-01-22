use std::fmt::Display;

use crate::lexer::pos::Pos;

#[derive(Debug)]
pub enum Error {
    UnexpectedToken {
        expected: Option<String>,
        value: String,
        pos: Pos,
    },
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnexpectedToken {
                expected,
                value,
                pos,
            } => {
                write!(f, "unexpected token '{}' at {}", value, pos)?;
                match expected {
                    Some(expected) => write!(f, ", expected {}", expected),
                    None => write!(f, ""),
                }
            }
        }
    }
}
