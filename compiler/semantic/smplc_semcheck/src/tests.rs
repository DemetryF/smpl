use std::rc::Rc;

use smplc_ast::{MakeSpanned, Span, Spanned};
use smplc_hir::{FunData, Pos, Type};
use smplc_lexer::Lexer;
use smplc_parse::{parse, TokenStream};

use crate::error::SemErrorKind;
use crate::sem_check;

macro_rules! semtest {
    ($code:literal => $error:expr) => {
        let token_stream = TokenStream::new(Lexer::new($code)).unwrap();
        let ast = parse(token_stream).unwrap();

        match sem_check(ast) {
            Err(error) => assert_eq!(error.kind, $error),
            _ => panic!("kaput"),
        }
    };
}

#[test]
pub fn non_existent_variable() {
    semtest![
        "fn main() { a; }" => SemErrorKind::NonExistentVariable("a")
    ];
}

#[test]
pub fn non_existent_function() {
    semtest![
        "fn main() { a(); }" => SemErrorKind::NonExistentFunction("a")
    ];
}

#[test]
pub fn redeclaring_variable() {
    semtest![
        "fn main() { let a: real; let a: real; }" => SemErrorKind::RedeclaringVariable {
            id: "a",
            first_declaration: Span::with_len(Pos::new(1, 17, 16), 1)
        }
    ];
}

#[test]
pub fn redeclaring_function() {
    semtest![
        "fn a() {} fn a() {}" => SemErrorKind::RedeclaringFunction {
            id: "a",
            first_declaration: Span::with_len(Pos::new(1, 4, 3), 1)
        }
    ];
}

#[test]
pub fn invalid_arguments() {
    semtest![
        "fn a() {}
         fn main() { a(1); }
        " => SemErrorKind::InvalidArgumentsCount {
            expected: 0,
            received: 1,
            fun_ref: Rc::new(FunData {
                id: Spanned::new("a", Span::with_len(Pos::new(1, 4, 3), 1 )),
                ret_ty: None,
                args_types: vec![]
            })
        }
    ];

    semtest![
        "fn a(b: real) {}
         fn main() { a(); }" => SemErrorKind::InvalidArgumentsCount {
            expected: 1,
            received: 0,
            fun_ref: Rc::new(FunData {
                id: "a".spanned(Span::with_len(Pos::new(1, 4, 3), 1)),
                ret_ty: None,
                args_types: vec![Type::Real],
            })
        }
    ];
}

#[test]
pub fn duplicate_args_names() {
    semtest![
        "fn a(b: real, b: real) {}" => SemErrorKind::DuplicateArgsNames("b")
    ];
}

#[test]
pub fn wrong_type() {
    semtest![
        "fn main() { let a: real = 1; }" => SemErrorKind::WrongType { received: Type::Int, expected: vec![Type::Real] }
    ];

    semtest![
        "fn main() { let a: real; a = 1; }" => SemErrorKind::WrongType { received: Type::Int, expected: vec![Type::Real] }
    ];

    semtest![
        "fn main() { if 1 {} }" => SemErrorKind::WrongType { received: Type::Int, expected: vec![Type::Bool] }
    ];

    semtest![
        "fn main() { while 1 {} }" => SemErrorKind::WrongType { received: Type::Int, expected: vec![Type::Bool] }
    ];

    semtest![
        "fn a(b: real) {} fn main() { a(1); }" => SemErrorKind::WrongType { received: Type::Int, expected: vec![Type::Real] }
    ];

    semtest![
        "fn a() -> real {} fn main() { let b: int = a(); }" => SemErrorKind::WrongType { received: Type::Real, expected: vec![Type::Int] }
    ];
}
