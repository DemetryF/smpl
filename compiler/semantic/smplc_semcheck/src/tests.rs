use std::rc::Rc;

use smplc_ast::Pos;
use smplc_hir::FunData;
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
        "fn main() { let a; let a; }" => SemErrorKind::RedeclaringVariable {
            id: "a",
            first_declaration: Pos::new(1, 17, 16)
        }
    ];
}

#[test]
pub fn redeclaring_function() {
    semtest![
        "fn a() {} fn a() {}" => SemErrorKind::RedeclaringFunction {
            id: "a",
            first_declaration: Pos::new(1, 4, 3)
        }
    ];
}

#[test]
pub fn invalid_arguments() {
    semtest![
        "fn a() {}
         fn main() { a(1); }
        " => SemErrorKind::InvalidArguments {
            expected_args_count: 0,
            received_args_count: 1,
            function_ref: Rc::new(FunData {
                declared_at: Pos::new(1, 4, 3),
                id: "a".into(),
                args_count: 0,
            })
        }
    ];

    semtest![
        "fn a(b) {} fn main() { a(); }" => SemErrorKind::InvalidArguments {
            expected_args_count: 1,
            received_args_count: 0,
            function_ref: Rc::new(FunData {
                declared_at: Pos::new(1, 4,  3),
                id: "a".into(),
                args_count: 1,
            })
        }
    ];
}

#[test]
pub fn duplicate_args_names() {
    semtest![
        "fn a(b, b) {}" => SemErrorKind::DuplicateArgsNames("b")
    ];
}
