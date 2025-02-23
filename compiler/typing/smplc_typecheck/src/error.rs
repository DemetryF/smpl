use smplc_ast::Span;

use crate::infer::TypeVar;

pub struct TypeError<'source> {
    pub kind: TypeErrorType<'source>,
    pub span: Span,
}

pub enum TypeErrorType<'source> {
    CouldNotInfer {
        var_id: &'source str,
        type_var: TypeVar,
    },
}

pub type TypeResult<'source, T> = Result<T, TypeError<'source>>;
