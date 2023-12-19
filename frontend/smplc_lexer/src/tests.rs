use smplc_ast::Literal;

use super::{Lexer, TokenValue};

macro_rules! lexer_test {
    (
        $code:expr; $($value:expr),* $(,)?
    ) => {
        let mut lexer = Lexer::new($code);
        $(
            assert_eq!(
                lexer.next_token().unwrap().value,
                $value
            );
        )*
    };
}

#[test]
pub fn numbers() {
    lexer_test![
        "
            384_400     /* common number */
            3.1415      /* double number */
            6.67e-11    /* exponential notation (minus) */
            6.022e+23   /* exponential notation (plus)  */ 
            1E10        /* exponential notation (no)    */
        ";

        TokenValue::Literal(Literal::Number(384400.0)),
        TokenValue::Literal(Literal::Number(3.1415)),
        TokenValue::Literal(Literal::Number(6.67e-11)),
        TokenValue::Literal(Literal::Number(6.022e+23)),
        TokenValue::Literal(Literal::Number(1e10))
    ];
}

#[test]
pub fn bool() {
    lexer_test![
        "true false";

        TokenValue::Literal(Literal::Bool(true)),
        TokenValue::Literal(Literal::Bool(false)),
    ];
}

#[test]
pub fn keywords() {
    lexer_test![
        "return while else let fn if";

        TokenValue::Return,
        TokenValue::While,
        TokenValue::Else,
        TokenValue::Let,
        TokenValue::Fn,
        TokenValue::If,
    ];
}

#[test]
pub fn id() {
    lexer_test![
        "_name12$";
        TokenValue::Id("_name12$".into()),
    ];
}

#[test]
pub fn specials() {
    lexer_test![
        "
            ;,(){}=
            != >= > <= < ==
            | & !
            + - * /
        ";
        TokenValue::Semicolon,
        TokenValue::Comma,
        TokenValue::LParen,
        TokenValue::RParen,
        TokenValue::LBrace,
        TokenValue::RBrace,
        TokenValue::Assign,

        TokenValue::Ne,
        TokenValue::Ge,
        TokenValue::Gt,
        TokenValue::Le,
        TokenValue::Lt,
        TokenValue::Eq,

        TokenValue::Or,
        TokenValue::And,
        TokenValue::Not,

        TokenValue::Plus,
        TokenValue::Minus,
        TokenValue::Star,
        TokenValue::Slash,
    ];
}
