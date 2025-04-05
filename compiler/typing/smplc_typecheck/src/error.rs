use std::fmt;

use smplc_ast::Span;

use crate::type_var::TypeVar;

pub struct TypeError<'source> {
    pub kind: TypeErrorKind<'source>,
    pub span: Span,
}

impl TypeError<'_> {
    pub fn mismatched_types(required: TypeVar, got: TypeVar, span: Span) -> Self {
        Self {
            kind: TypeErrorKind::MismatchedTypes { required, got },
            span,
        }
    }
}

pub enum TypeErrorKind<'source> {
    CouldNotInfer {
        var_id: &'source str,
        type_var: TypeVar,
    },
    MismatchedTypes {
        required: TypeVar,
        got: TypeVar,
    },
}

impl fmt::Display for TypeErrorKind<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TypeErrorKind::CouldNotInfer { var_id, type_var } => {
                write!(f, "couldn't infer the type of \"{var_id}\" variable. Inferred type variable: {type_var:?}")
            }

            TypeErrorKind::MismatchedTypes { required, got } => {
                write!(f, "mismatched types: required {required}, got {got}")
            }
        }
    }
}

pub type TypeResult<'source, T> = Result<T, TypeError<'source>>;
