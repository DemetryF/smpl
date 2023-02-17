use derive_more::Constructor;
use std::fmt::Display;

use crate::{
    ast::Id,
    token::{Pos, TokenValue},
};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Constructor, Debug)]
pub struct Error {
    pub kind: ErrorKind,
    pub pos: Pos,
}

#[derive(Debug)]
pub enum ErrorKind {
    UnexpectedChar(char),
    UnexpectedToken(TokenValue),

    NonExistingVariable(String),
    NonExistingFunction(String),
    ReDeclaringVariable {
        name: String,
        defined_at: Pos,
    },
    InvalidArgumentsCount {
        expected_args_count: usize,
        received_args_count: usize,
        function_id: Id,
    },
    DuplicateFunctionArgs(String),
}

impl Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorKind::UnexpectedToken(value) => {
                write!(f, "unexpected token '{value}'")
            }
            ErrorKind::UnexpectedChar(value) => {
                write!(f, "unexpected char '{value}'")
            }
            ErrorKind::NonExistingFunction(id) => write!(f, "\"{id}\" is not defined"),
            ErrorKind::NonExistingVariable(id) => write!(f, "\"{id}\" is not defined"),
            ErrorKind::ReDeclaringVariable { name, .. } => {
                write!(f, "\"{name}\" is already declared")
            }
            ErrorKind::InvalidArgumentsCount {
                expected_args_count,
                received_args_count,
                ..
            } => write!(
                f,
                "expected {expected_args_count} args, received {received_args_count}"
            ),
            ErrorKind::DuplicateFunctionArgs(id) => write!(f, "duplicate arg \"{id}\""),
        }
    }
}
