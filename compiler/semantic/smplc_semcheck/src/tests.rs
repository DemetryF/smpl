use std::rc::Rc;

use smplc_hir::{FunData, Pos, Type};
use smplc_lexer::lex;
use smplc_parse::{parse, TokenStream};

use crate::error::SemErrorKind;
use crate::sem_check;

macro_rules! semtest {
    ($code:literal => $error:expr) => {
        let tokens = lex($code).unwrap();
        let token_stream = TokenStream::new(tokens);
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
            name: "a",
            first_declaration: Pos::new(1, 17, 16)
        }
    ];
}

#[test]
pub fn redeclaring_function() {
    semtest![
        "fn a() {} fn a() {}" => SemErrorKind::RedeclaringFunction {
            name: "a",
            first_declaration: Pos::new(1, 4, 3)
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
                declared_at: Pos::new(1, 4, 3),
                id: "a".into(),
                args: vec![],
                ret_ty: None
            })
        }
    ];

    semtest![
        "fn a(b: real) {} fn main() { a(); }" => SemErrorKind::InvalidArgumentsCount {
            expected: 1,
            received: 0,
            fun_ref: Rc::new(FunData {
                declared_at: Pos::new(1, 4, 3),
                id: "a".into(),
                args: vec![Type::Real],
                ret_ty: None
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
