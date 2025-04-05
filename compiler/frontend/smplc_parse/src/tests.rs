use smplc_ast::*;
use smplc_lexer::Lexer;

use crate::{Parse, TokenStream};

macro_rules! parse_test {
    ($code:expr; $decl:pat $(=> $block:block)?) => {{
        let mut token_stream = TokenStream::new(Lexer::new($code)).unwrap();

        let decl = Declaration::parse(&mut token_stream).unwrap();

        match decl {
            $decl => {
                $(
                    $block
                )?
            }
            _ => panic!(),
        }
    }};
}

macro_rules! stmt_test {
    ($code:expr; $stmt:pat $(=> $block:block)?) => {{
        let mut token_stream = TokenStream::new(Lexer::new($code)).unwrap();

        let stmt = Statement::parse(&mut token_stream).unwrap();

        match stmt {
            $stmt => {
                $(
                    $block
                )?
            }
            _ => panic!(),
        }
    }};
}

macro_rules! expr_test {
    ($code:expr; $expr:pat $(=> $block:block)?) => {{
        let mut token_stream = TokenStream::new(Lexer::new($code)).unwrap();

        let expr = Spanned::<Expr>::parse(&mut token_stream).unwrap().0;

        match expr {
            $expr => {
                $(
                    $block
                )?
            }
            _ => panic!(),
        }
    }};
}

#[test]
pub fn declare_statement() {
    stmt_test!(
        "\
let a: int;
        ";
        Statement::Declare(DeclareStatement {
            id: Spanned("a", _),
            ty: Some(Spanned("int", _)),
            value: None,
        })
    );

    stmt_test!(
        "\
let a: real = a;
        ";
        Statement::Declare(DeclareStatement {
            id: Spanned("a", _),
            ty: Some(Spanned("real", _)),
            value: Some(Spanned(Expr::Atom(Atom::Id(
                Spanned("a", _)
            )), _)),
        })
    );
}

#[test]
pub fn expr_statement() {
    stmt_test!(
        "\
a;
        ";
        Statement::Expr(ExprStatement::Expr(Spanned(Expr::Atom(Atom::Id(Spanned("a", _))), _)))
    );
}

#[test]
pub fn function_statement() {
    parse_test!(
        "\
fn name() {}
        ";
        Declaration::Function(FunctionDeclaration {
            id: Spanned("name", _),
            args,
            ret_ty: None,
            body: Block { statements },
        }) => {
            assert!(matches!(args.as_slice(), []));
            assert!(matches!(statements.as_slice(), []));
        }
    );

    parse_test!(
        "\
fn name(a: real) -> real {}
        ";
        Declaration::Function(FunctionDeclaration {
            id: Spanned("name", _),
            args,
            ret_ty: Some(Spanned("real", _)),
            body: Block { statements },
        }) => {
            assert!(matches!(args.as_slice(), [
                FunctionArg {
                    id:  Spanned("a", _),
                    ty: Spanned("real", _),
                }
            ]));

            assert!(matches!(statements.as_slice(), []));
        }
    );

    parse_test!(
        "\
fn name(a: bool, b: bool) -> bool {}
        ";
        Declaration::Function(FunctionDeclaration {
            id: Spanned("name", _),
            args,
            ret_ty: Some(Spanned("bool", _)),
            body: Block { statements },
        }) => {
            assert!(matches!(args.as_slice(), [
                FunctionArg {
                    id: Spanned("a", _),
                    ty: Spanned("bool", _),
                },
                FunctionArg {
                    id: Spanned("b", _),
                    ty: Spanned("bool", _),
                }
            ]));

            assert!(matches!(statements.as_slice(), []))
        }
    );
}

#[test]
pub fn if_statement() {
    stmt_test!(
        "\
if a { }
        ";

        Statement::If(IfStatement {
            cond: Spanned(Expr::Atom(Atom::Id(Spanned("a", _))), _),
            body: Block { statements },
            else_body: None,
        }) => {
            assert!(matches!(statements.as_slice(), []))
        }
    );

    stmt_test!(
        "\
if a { }
else { }
        ";

        Statement::If(IfStatement {
            cond: Spanned(Expr::Atom(Atom::Id(Spanned("a", _))), _),
            body: Block { statements: body },
            else_body: Some(Block { statements: else_stmts }),
        }) => {
            assert!(matches!(body.as_slice(), []));
            assert!(matches!(else_stmts.as_slice(), []));
        }
    );
}

#[test]
pub fn return_statement() {
    parse_test!(
        "\
fn name() {
    return;
}
        ";
        Declaration::Function(FunctionDeclaration {
            id: Spanned("name", _),
            args,
            ret_ty: None,
            body: Block {
                statements,
            },
        }) => {
            assert!(matches!(args.as_slice(), []));
            assert!(matches!(statements.as_slice(), [
                Statement::Return(ReturnStatement { value: None })
            ]));
        }
    );

    parse_test!(
        "\
fn name() {
    return a;
}
        ";

        Declaration::Function(FunctionDeclaration {
            id: Spanned("name", _),
            args,
            ret_ty: None,
            body: Block {
                statements,
            },
        }) => {
            assert!(matches!(args.as_slice(), []));
            assert!(matches!(statements.as_slice(), [
                Statement::Return(ReturnStatement {
                    value: Some(Spanned(Expr::Atom(Atom::Id(Spanned("a", _))), _)),
                })
            ]));
        }
    );
}

#[test]
pub fn while_statement() {
    stmt_test!(
        "\
while a {}
        ";

        Statement::While(WhileStatement {
            cond: Spanned(Expr::Atom(Atom::Id(Spanned("a", _))), _),
            body: Block { statements },
        }) => {
            assert!(matches!(statements.as_slice(), []));
        }
    );
}

#[test]
pub fn expr_call() {
    expr_test!(
        "call();";
        Expr::Call(Call { id: Spanned("call", _), args }) => {
            assert!(matches!(args, _));
        }
    );

    expr_test!(
        "call(1);";
        Expr::Call(Call {
            id: Spanned("call", _),
            args,
        }) => {
            assert!(matches!(args.as_slice(), [
                Spanned(Expr::Atom(Atom::Literal(Literal { value: "1", .. })), _)
            ]))
        }
    );

    expr_test!(
        "call(1, 2);";
        Expr::Call(Call {
            id: Spanned("call", _),
            args,
        }) => {
            assert!(matches!(args.as_slice(), [
                Spanned(Expr::Atom(Atom::Literal(Literal { value: "1",.. })), _),
                Spanned(Expr::Atom(Atom::Literal(Literal { value: "2", .. })), _)
            ]))
        }
    );
}
