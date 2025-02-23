use std::fmt;

use smplc_ast::Span;

use crate::infer::TypeVar;

pub struct TypeError<'source> {
    pub kind: TypeErrorKind<'source>,
    pub span: Span,
}

pub enum TypeErrorKind<'source> {
    CouldNotInfer {
        var_id: &'source str,
        type_var: TypeVar,
    },
}

impl fmt::Display for TypeErrorKind<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TypeErrorKind::CouldNotInfer { var_id, type_var } => {
                write!(f, "couldn't infer the type of \"{var_id}\" variable. Inferred type variable: {type_var:?}")
            }
        }
    }
}

pub type TypeResult<'source, T> = Result<T, TypeError<'source>>;
