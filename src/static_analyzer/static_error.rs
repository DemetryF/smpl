use crate::{ast::Id, lexer::Pos};
use derive_more::Constructor;
use std::fmt::Display;

#[derive(Constructor)]
pub struct StaticError {
    pub kind: StaticErrorKind,
    pub pos: Pos,
}

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
            Self::NonExistingFunction(id) => write!(f, "\"{id}\" is not defined"),
            Self::NonExistingVariable(id) => write!(f, "\"{id}\" is not defined"),
            Self::ReDeclaringVariable { name, .. } => write!(f, "\"{name}\" is already declared"),
            Self::InvalidArgumentsCount {
                expected_args_count,
                received_args_count,
                ..
            } => write!(
                f,
                "expected {expected_args_count} args, received {received_args_count}"
            ),
            Self::DuplicateFunctionArgs(id) => write!(f, "duplicate arg \"{id}\""),
        }
    }
}
