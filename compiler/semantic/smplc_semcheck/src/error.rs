use std::fmt;

use smplc_ast as ast;
use smplc_hir::{FunRef, Pos, Type};

pub type SemResult<'source, T> = Result<T, SemError<'source>>;

#[derive(Debug)]
pub struct SemError<'source> {
    pub kind: SemErrorKind<'source>,
    pub pos: Pos,
}

#[derive(Debug, PartialEq)]
pub enum SemErrorKind<'source> {
    NonExistentFunction(&'source str),
    NonExistentVariable(&'source str),

    RedeclaringVariable {
        name: &'source str,
        first_declaration: Pos,
    },

    RedeclaringFunction {
        name: &'source str,
        first_declaration: Pos,
    },

    InvalidArgumentsCount {
        expected: usize,
        received: usize,
        fun_ref: FunRef,
    },

    DuplicateArgsNames(&'source str),

    WrongType {
        received: Type,
        expected: Vec<Type>,
    },
}

impl<'source> SemError<'source> {
    pub fn non_existent_variable(id: ast::Id<'source>) -> Self {
        let ast::Id { name, pos } = id;

        Self {
            kind: SemErrorKind::NonExistentVariable(name),
            pos,
        }
    }

    pub fn non_existent_function(id: ast::Id<'source>) -> Self {
        let ast::Id { name, pos } = id;

        Self {
            kind: SemErrorKind::NonExistentFunction(name),
            pos,
        }
    }

    pub fn redeclaring_variable(id: ast::Id<'source>, first_declaration: Pos) -> Self {
        let ast::Id { name, pos } = id;

        let kind = SemErrorKind::RedeclaringVariable {
            name,
            first_declaration,
        };

        Self { kind, pos }
    }

    pub fn redeclaring_function(id: ast::Id<'source>, first_declaration: Pos) -> Self {
        let ast::Id { name, pos } = id;

        let kind = SemErrorKind::RedeclaringFunction {
            name,
            first_declaration,
        };

        Self { kind, pos }
    }

    pub fn invalid_arguments(pos: Pos, expected: usize, received: usize, fun_ref: FunRef) -> Self {
        let kind = SemErrorKind::InvalidArgumentsCount {
            expected,
            received,
            fun_ref,
        };

        Self { kind, pos }
    }

    pub fn duplicate_args_names(id: ast::Id<'source>) -> Self {
        let ast::Id { name, pos } = id;

        Self {
            kind: SemErrorKind::DuplicateArgsNames(name),
            pos,
        }
    }

    pub fn wrong_ty(received: Type, expected: Vec<Type>) -> Self {
        Self {
            kind: SemErrorKind::WrongType { received, expected },
            pos: Pos::default(),
        }
    }
}

impl fmt::Display for SemErrorKind<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SemErrorKind::RedeclaringVariable {
                name,
                first_declaration,
            } => write!(
                f,
                "variable \"{name}\" is already declared at {first_declaration}"
            ),

            SemErrorKind::RedeclaringFunction {
                name,
                first_declaration,
            } => write!(
                f,
                "function \"{name}\" is already declared at {first_declaration}"
            ),

            SemErrorKind::InvalidArgumentsCount {
                expected,
                received,
                fun_ref,
            } => write!(
                f,
                "function \"{}\" takes {expected}, but received {received}",
                fun_ref.id
            ),

            SemErrorKind::NonExistentVariable(name) => {
                write!(f, "variable \"{name}\" is not defined")
            }

            SemErrorKind::NonExistentFunction(name) => {
                write!(f, "function \"{name}\" is not defined")
            }

            SemErrorKind::DuplicateArgsNames(name) => {
                write!(f, "two arguments with same name: {name}")
            }

            SemErrorKind::WrongType { received, expected } => {
                write!(f, "wrong type: received {received}")?;

                match expected.as_slice() {
                    [] => Ok(()),

                    [ty] => write!(f, "but expected {ty}"),

                    [first, middle @ .., last] => {
                        write!(f, "but expected {first}")?;

                        for ty in middle {
                            write!(f, ", {ty}")?;
                        }

                        write!(f, "or {last}")
                    }
                }
            }
        }
    }
}
