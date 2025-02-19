use std::fmt;

use smplc_ast as ast;
use smplc_ast::Span;

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
    DuplicateArgsNames(&'source str),

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
        fun_id: &'source str,
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

    pub fn invalid_arguments_count(id: ast::Id<'source>, expected: usize, received: usize) -> Self {
        let kind = SemErrorKind::InvalidArgumentsCount {
            expected,
            received,
            fun_id: id.0,
        };

        Self {
            kind,
            span: id.span(),
        }
    }

    pub fn duplicate_args_names(id: ast::Id<'source>) -> Self {
        let ast::Spanned(id, span) = id;

        Self {
            kind: SemErrorKind::DuplicateArgsNames(id),
            span,
        }
    }
}

impl fmt::Display for SemErrorKind<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SemErrorKind::RedeclaringVariable {
                id,
                first_declaration,
            } => write!(
                f,
                "variable \"{id}\" is already declared at {first_declaration}"
            ),

            SemErrorKind::RedeclaringFunction {
                id,
                first_declaration,
            } => {
                write!(
                    f,
                    "function \"{id}\" is already declared at {first_declaration}"
                )
            }

            SemErrorKind::InvalidArgumentsCount {
                expected,
                received,
                fun_id: fun_if,
            } => {
                write!(
                    f,
                    "function \"{fun_if}\" takes {expected}, but received {received}"
                )
            }

            SemErrorKind::NonExistentVariable(name) => {
                write!(f, "variable \"{name}\" is not defined")
            }

            SemErrorKind::NonExistentFunction(name) => {
                write!(f, "function \"{name}\" is not defined")
            }

            SemErrorKind::DuplicateArgsNames(name) => {
                write!(f, "two arguments with same name: {name}")
            }
        }
    }
}
