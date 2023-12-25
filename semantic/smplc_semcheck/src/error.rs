use std::fmt;

use smplc_ast::Pos;
use smplc_hir::FunRef;

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
        id: &'source str,
        first_declaration: Pos,
    },

    RedeclaringFunction {
        id: &'source str,
        first_declaration: Pos,
    },

    InvalidArguments {
        expected_args_count: usize,
        received_args_count: usize,
        function_ref: FunRef,
    },

    DuplicateArgsNames(&'source str),
}

impl<'source> SemError<'source> {
    pub fn non_existent_variable(id: smplc_ast::Id<'source>) -> Self {
        let smplc_ast::Id { id, pos } = id;

        Self {
            kind: SemErrorKind::NonExistentVariable(id),
            pos,
        }
    }

    pub fn non_existent_function(id: smplc_ast::Id<'source>) -> Self {
        let smplc_ast::Id { id, pos } = id;

        Self {
            kind: SemErrorKind::NonExistentFunction(id),
            pos,
        }
    }

    pub fn redeclaring_variable(id: smplc_ast::Id<'source>, first_declaration: Pos) -> Self {
        let smplc_ast::Id { id, pos } = id;

        let kind = SemErrorKind::RedeclaringVariable {
            id,
            first_declaration,
        };

        Self { kind, pos }
    }

    pub fn redeclaring_function(id: smplc_ast::Id<'source>, first_declaration: Pos) -> Self {
        let smplc_ast::Id { id, pos } = id;

        let kind = SemErrorKind::RedeclaringFunction {
            id,
            first_declaration,
        };

        Self { kind, pos }
    }

    pub fn invalid_arguments(
        pos: Pos,
        expected_args_count: usize,
        received_args_count: usize,
        function_ref: FunRef,
    ) -> Self {
        let kind = SemErrorKind::InvalidArguments {
            expected_args_count,
            received_args_count,
            function_ref,
        };

        Self { kind, pos }
    }

    pub fn duplicate_args_names(id: smplc_ast::Id<'source>) -> Self {
        let smplc_ast::Id { id, pos } = id;

        Self {
            kind: SemErrorKind::DuplicateArgsNames(id),
            pos,
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
            } => write!(
                f,
                "function \"{id}\" is already declared at {first_declaration}"
            ),

            SemErrorKind::InvalidArguments {
                expected_args_count,
                received_args_count,
                function_ref,
            } => write!(
                f,
                "function \"{}\" takes {expected_args_count}, but received {received_args_count}",
                function_ref.id.0
            ),

            SemErrorKind::NonExistentVariable(id) => write!(f, "variable \"{id}\" is not defined"),
            SemErrorKind::NonExistentFunction(id) => write!(f, "function \"{id}\" is not defined"),

            SemErrorKind::DuplicateArgsNames(id) => {
                write!(f, "two arguments with same name: {id}")
            }
        }
    }
}
