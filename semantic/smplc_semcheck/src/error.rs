use smplc_ast::Pos;

pub type SemResult<'source, T> = Result<T, SemError<'source>>;

pub struct SemError<'source> {
    pub kind: SemErrorKind<'source>,
    pub pos: Pos,
}

pub enum SemErrorKind<'source> {
    NonExistentFunction(&'source str),
    NonExistentVariable(&'source str),

    RedeclaringVariable {
        id: &'source str,
        first_declaration: Pos,
    },

    ReDeclaringFunction {
        id: &'source str,
        first_declaration: Pos,
    },

    InvalidArguments {
        expected_args_count: usize,
        received_args_count: usize,
        function_id: &'source str,
    },

    DuplicateArgsNames(&'source str),
}
