use std::fmt::Display;

use derive_more::Constructor;

use smplc_ast::{Id, Pos};

use crate::scopes::{Function, Variable};

#[derive(Debug, Constructor)]
pub struct Error {
    pub kind: ErrorKind,
    pub pos: Pos,
}

#[derive(Debug)]
pub enum ErrorKind {
    ReDeclaringVariable {
        id: String,
        first_declaration: Pos,
    },

    ReDeclaringFunction {
        id: String,
        first_declaration: Pos,
    },

    InvalidArgumentsCount {
        expected_args_count: usize,
        received_args_count: usize,
        function_id: Id,
    },

    NonExistentVariable(String),
    NonExistentFunction(String),

    DuplicateFunctionArgs(String),

    ExpectedLValue,
    UnexpectedAssignment,
}

impl Error {
    pub fn non_existent_variable(id: Id) -> Self {
        let kind = ErrorKind::NonExistentVariable(id.id);

        Self::new(kind, id.pos)
    }

    pub fn non_existent_function(id: Id) -> Self {
        let kind = ErrorKind::NonExistentFunction(id.id);

        Self::new(kind, id.pos)
    }

    pub fn redeclaring_variable(id: Id, variable: Variable) -> Self {
        let kind = ErrorKind::ReDeclaringVariable {
            id: id.id,
            first_declaration: variable.defined_at,
        };

        Self::new(kind, id.pos)
    }

    pub fn redeclaring_function(id: Id, function: Function) -> Self {
        let kind = ErrorKind::ReDeclaringFunction {
            id: id.id,
            first_declaration: function.defined_at,
        };

        Self::new(kind, id.pos)
    }

    pub fn duplicate_function_args(id: Id) -> Self {
        let kind = ErrorKind::DuplicateFunctionArgs(id.id);

        Self::new(kind, id.pos)
    }

    pub fn expected_lvalue() -> Self {
        Self::new(ErrorKind::ExpectedLValue, Pos::default())
    }

    pub fn unexpected_assignment() -> Self {
        Self::new(ErrorKind::UnexpectedAssignment, Pos::default())
    }
}

impl Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorKind::ReDeclaringVariable {
                id,
                first_declaration,
            } => write!(
                f,
                "variable \"{id}\" is already declared at {first_declaration}"
            ),

            ErrorKind::ReDeclaringFunction {
                id,
                first_declaration,
            } => write!(
                f,
                "function \"{id}\" is already declared at {first_declaration}"
            ),

            ErrorKind::InvalidArgumentsCount {
                expected_args_count,
                received_args_count,
                function_id,
            } => write!(f, "function \"{function_id}\" takes {expected_args_count}, but received {received_args_count}"),

            ErrorKind::NonExistentVariable(id) => write!(f, "variable \"{id}\" is not defined"),
            ErrorKind::NonExistentFunction(id) => write!(f, "function \"{id}\" is not defined"),

            ErrorKind::DuplicateFunctionArgs(id) => write!(f, "two arguments with same name: {id}"),

            ErrorKind::ExpectedLValue => write!(f, "expected lvalue"), 
            ErrorKind::UnexpectedAssignment => write!(f, "unexpected assignment"),        
        }
    }
}
