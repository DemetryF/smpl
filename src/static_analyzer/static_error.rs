use std::fmt::Display;

use derive_more::Constructor;

use crate::{lexer::pos::Pos, parser::ast::Id};

#[derive(Debug, Constructor)]
pub struct StaticError {
    pub kind: StaticErrorKind,
    pub pos: Pos,
}

#[derive(Debug)]
pub enum StaticErrorKind {
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

impl Display for StaticErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NonExistingFunction(id) => write!(f, "\"{}\" is not defined", id),
            Self::NonExistingVariable(id) => write!(f, "\"{}\" is not defined", id),
            Self::ReDeclaringVariable { name, .. } => write!(f, "\"{}\" is already declared", name),
            Self::InvalidArgumentsCount {
                expected_args_count,
                received_args_count,
                ..
            } => write!(
                f,
                "expected {} args, received {}",
                expected_args_count, received_args_count
            ),
            Self::DuplicateFunctionArgs(id) => write!(f, "duplicate arg \"{}\"", id),
        }
    }
}
