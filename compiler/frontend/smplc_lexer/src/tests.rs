use smplc_ast::Type;

use crate::{Lexer, TokenTag};

macro_rules! lexer_test {
    (
        $code:expr;
        $($tag:expr $(, $value:literal)?;)*
    ) => {
        let mut lexer = Lexer::new($code);
        $(
            let token = lexer.next_token().unwrap();

            assert_eq!(token.tag, $tag);

            $(
                assert_eq!(token.value, $value);
            )?
        )*
    };
}

#[test]
fn numbers() {
    lexer_test![
        "
            384_400     /* common number */
            3.1415      /* double number */
            6.67e-11    /* exponential notation (minus) */
            6.022e+23   /* exponential notation (plus)  */ 
            1E10        /* exponential notation (no)    */
        ";

        TokenTag::Literal(Type::Int), "384_400";
        TokenTag::Literal(Type::Real), "3.1415";
        TokenTag::Literal(Type::Real), "6.67e-11";
        TokenTag::Literal(Type::Real), "6.022e+23";
        TokenTag::Literal(Type::Real), "1E10";
    ];
}

#[test]
fn bool() {
    lexer_test![
        "true false";

        TokenTag::Literal(Type::Bool), "true";
        TokenTag::Literal(Type::Bool), "false";
    ];
}

#[test]
fn keywords() {
    lexer_test![
        "return while else let fn if continue break const int real bool";

        TokenTag::Return;
        TokenTag::While;
        TokenTag::Else;
        TokenTag::Let;
        TokenTag::Fn;
        TokenTag::If;
        TokenTag::Continue;
        TokenTag::Break;
        TokenTag::Const;

        TokenTag::Type(Type::Int);
        TokenTag::Type(Type::Real);
        TokenTag::Type(Type::Bool);
    ];
}

#[test]
fn id() {
    lexer_test![
        "_name12$";
        TokenTag::Id, "_name12$";
    ];
}

#[test]
fn specials() {
    lexer_test![
        "
            ;,(){}=
            != >= > <= < ==
            | & !
            + - * /
        ";
        TokenTag::Semicolon;
        TokenTag::Comma;
        TokenTag::LParen;
        TokenTag::RParen;
        TokenTag::LBrace;
        TokenTag::RBrace;
        TokenTag::Assign;

        TokenTag::Ne;
        TokenTag::Ge;
        TokenTag::Gt;
        TokenTag::Le;
        TokenTag::Lt;
        TokenTag::Eq;

        TokenTag::Or;
        TokenTag::And;
        TokenTag::Not;

        TokenTag::Plus;
        TokenTag::Minus;
        TokenTag::Star;
        TokenTag::Slash;
    ];
}
