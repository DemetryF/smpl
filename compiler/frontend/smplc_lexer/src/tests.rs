use crate::token::{Literal, TokenValue};
use crate::Lexer;

macro_rules! lexer_test {
    (
        $code:expr;

        [
            $($value:expr),* $(,)?
        ]
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
pub fn nums() {
    lexer_test![
        "
            384_400     /* common number */
            3.1415      /* double number */
            6.67e-11    /* exponential notation (minus)   */
            6.022e+23   /* exponential notation (plus)    */ 
            1E10        /* exponential notation (no sign) */
        ";

        [
            TokenValue::Literal(Literal::Num(384400.0)),
            TokenValue::Literal(Literal::Num(3.1415)),
            TokenValue::Literal(Literal::Num(6.67e-11)),
            TokenValue::Literal(Literal::Num(6.022e+23)),
            TokenValue::Literal(Literal::Num(1e10))
        ]
    ];
}

#[test]
pub fn bools() {
    lexer_test![
        "true false";

        [
            TokenValue::Literal(Literal::Bool(true)),
            TokenValue::Literal(Literal::Bool(false))
        ]
    ];
}

#[test]
pub fn keywords() {
    lexer_test![
        "continue return break while else let fn if";

        [
            TokenValue::Continue,
            TokenValue::Return,
            TokenValue::Break,
            TokenValue::While,
            TokenValue::Else,
            TokenValue::Let,
            TokenValue::Fn,
            TokenValue::If
        ]
    ];
}

#[test]
pub fn idents() {
    lexer_test![
        "_name12$ Break";

        [
            TokenValue::Ident("_name12$".into()),
            TokenValue::Ident("Break".into())
        ]
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

        [
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
            TokenValue::Asterisk,
            TokenValue::Slash
        ]
    ];
}
