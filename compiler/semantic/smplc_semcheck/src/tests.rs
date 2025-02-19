use smplc_ast::Span;
use smplc_hir::Pos;
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
            fun_id: "a"
        }
    ];

    semtest![
        "fn a(b: real) {}
         fn main() { a(); }" => SemErrorKind::InvalidArgumentsCount {
            expected: 1,
            received: 0,
            fun_id: "a"
        }
    ];
}

#[test]
pub fn duplicate_args_names() {
    semtest![
        "fn a(b: real, b: real) {}" => SemErrorKind::DuplicateArgsNames("b")
    ];
}
