use super::{Lexer, Literal, TokenValue};

macro_rules! lexer_test {
    (
        $lexer:ident; $($value:expr),* $(,)?
    ) => {
        $(
            assert_eq!(
                $lexer.next_token().unwrap().value,
                $value
            );
        )*
    };
}

#[test]
pub fn test_numbers() {
    let mut lexer = Lexer::new(
        "
        384_400     /* common number */
        3.1415      /* double number */
        6.67e-11    /* exponential notation (minus) */
        6.022e+23   /* exponential notation (plus)  */ 
        1E10        /* exponential notation (no)    */
        ",
    );

    lexer_test![
        lexer;
        TokenValue::Literal(Literal::Number(384400.0)),
        TokenValue::Literal(Literal::Number(3.1415)),
        TokenValue::Literal(Literal::Number(6.67e-11)),
        TokenValue::Literal(Literal::Number(6.022e+23)),
        TokenValue::Literal(Literal::Number(1e10))
    ];
}

#[test]
pub fn test_bool() {
    let mut lexer = Lexer::new("true false");

    lexer_test![
        lexer;
        TokenValue::Literal(Literal::Bool(true)),
        TokenValue::Literal(Literal::Bool(false)),
    ];
}

#[test]
pub fn test_keywords() {
    let mut lexer = Lexer::new("return while else let fn if");

    lexer_test![
        lexer;
        TokenValue::Return,
        TokenValue::While,
        TokenValue::Else,
        TokenValue::Let,
        TokenValue::Fn,
        TokenValue::If,
    ];
}

#[test]
pub fn test_id() {
    let mut lexer = Lexer::new("_name12$");

    lexer_test![
        lexer;
        TokenValue::Id("_name12$".into()),
    ];
}

#[test]
pub fn test_specials() {
    let mut lexer = Lexer::new(
        "
            ;,(){}=
            != >= > <= < ==
            | & !
            + - * /
        ",
    );

    lexer_test![
        lexer;
        TokenValue::Semicolon,
        TokenValue::Comma,
        TokenValue::LParen,
        TokenValue::RParen,
        TokenValue::LBrace,
        TokenValue::RBrace,
        TokenValue::Assignment,

        TokenValue::NotEqual,
        TokenValue::GreaterOrEqual,
        TokenValue::Greater,
        TokenValue::LessOrEqual,
        TokenValue::Less,
        TokenValue::Equal,

        TokenValue::Or,
        TokenValue::And,
        TokenValue::Not,

        TokenValue::Plus,
        TokenValue::Minus,
        TokenValue::Star,
        TokenValue::Slash,
    ];
}
