use std::fmt;

use smplc_ast::{self as ast, Span};
use smplc_hir::{FunRef, Type};

pub type SemResult<'source, T> = Result<T, SemError<'source>>;

#[derive(Debug)]
pub struct SemError<'source> {
    pub kind: SemErrorKind<'source>,
    pub span: Span,
}

#[derive(Debug, PartialEq)]
pub enum SemErrorKind<'source> {
    NonExistentFunction(&'source str),
    NonExistentVariable(&'source str),

    RedeclaringVariable {
        id: &'source str,
        first_declaration: Span,
    },

    RedeclaringFunction {
        id: &'source str,
        first_declaration: Span,
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
        let ast::Spanned(id, span) = id;

        Self {
            kind: SemErrorKind::NonExistentVariable(id),
            span,
        }
    }

    pub fn non_existent_function(id: ast::Id<'source>) -> Self {
        let ast::Spanned(id, span) = id;

        Self {
            kind: SemErrorKind::NonExistentFunction(id),
            span,
        }
    }

    pub fn redeclaring_variable(id: ast::Id<'source>, first_declaration: Span) -> Self {
        let ast::Spanned(id, span) = id;

        let kind = SemErrorKind::RedeclaringVariable {
            id,
            first_declaration,
        };

        Self { kind, span }
    }

    pub fn redeclaring_function(id: ast::Id<'source>, first_declaration: Span) -> Self {
        let ast::Spanned(id, span) = id;

        let kind = SemErrorKind::RedeclaringFunction {
            id,
            first_declaration,
        };

        Self { kind, span }
    }

    pub fn invalid_arguments_count(
        span: Span,
        expected: usize,
        received: usize,
        fun_ref: FunRef,
    ) -> Self {
        let kind = SemErrorKind::InvalidArgumentsCount {
            expected,
            received,
            fun_ref,
        };

        Self { kind, span }
    }

    pub fn duplicate_args_names(id: ast::Id<'source>) -> Self {
        let ast::Spanned(id, span) = id;

        Self {
            kind: SemErrorKind::DuplicateArgsNames(id),
            span,
        }
    }

    pub fn wrong_ty(span: Span, received: Type, expected: Vec<Type>) -> Self {
        let kind = SemErrorKind::WrongType { received, expected };

        Self { kind, span }
    }
}

impl fmt::Display for SemErrorKind<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SemErrorKind::RedeclaringVariable {
                id: name,
                first_declaration,
            } => write!(
                f,
                "variable \"{name}\" is already declared at {first_declaration}"
            ),

            SemErrorKind::RedeclaringFunction {
                id: name,
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
                &fun_ref.id
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

                    [ty] => write!(f, ", but expected {ty}"),

                    [first, middle @ .., last] => {
                        write!(f, ", but expected {first}")?;

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
