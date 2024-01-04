use smplc_ast::*;
use smplc_lexer::lex;

use crate::{Parse, TokenStream};

macro_rules! parse_test {
    ($code:expr; $stmt:expr) => {
        assert_eq!(
            Declaration::parse(&mut TokenStream::new(lex($code).unwrap())).unwrap(),
            $stmt
        );
    };
}

macro_rules! statement_test {
    ($code:expr; $stmt:expr) => {
        assert_eq!(
            Statement::parse(&mut TokenStream::new(lex($code).unwrap())).unwrap(),
            $stmt
        );
    };
}

macro_rules! expr_test {
    ($code:expr; $expr:expr) => {{
        let mut token_stream = TokenStream::new(lex($code).unwrap());
        assert_eq!(Expr::parse(&mut token_stream).unwrap(), $expr);
    }};
}

#[test]
pub fn declare_statement() {
    statement_test!(
        "\
let a;
        ";
        Statement::Declare(DeclareStatement {
            id: Id::new("a".into(), Pos::new(1, 5, 4)),
            init_expr: None,
        })
    );

    statement_test!(
        "\
let a = a;
        ";
        Statement::Declare(DeclareStatement {
            id: Id::new("a".into(), Pos::new(1, 5,  4)),
            init_expr: Some(Expr::Atom(Atom::Id(Id::new(
                "a".into(),
                Pos::new(1, 9, 8),
            )))),
        })
    );
}

#[test]
pub fn expr_statement() {
    statement_test!(
        "\
a;
        ";
        Statement::Expr(ExprStatement::Expr(Expr::Atom(Atom::Id(Id::new(
            "a".into(),
            Pos::new(1, 1, 0),
        )))))
    );
}

#[test]
pub fn function_statement() {
    parse_test!(
        "\
fn name() {}
        ";
        Declaration::Function(FunctionDeclaration {
            id: Id::new("name".into(), Pos::new(1, 4, 3)),
            args: vec![],
            body: Block { stmts: vec![] },
        })
    );

    parse_test!(
        "\
fn name(a) {}
        ";
        Declaration::Function(FunctionDeclaration {
            id: Id::new("name".into(), Pos::new(1, 4, 3)),
            args: vec![
                Id::new("a".into(), Pos::new(1, 9, 8))
            ],
            body: Block { stmts: vec![] },
        })
    );

    parse_test!(
        "\
fn name(a, b) {}
        ";
        Declaration::Function(FunctionDeclaration {
            id: Id::new("name".into(), Pos::new(1, 4,  3)),
            args: vec![
                Id::new("a".into(), Pos::new(1, 9, 8)),
                Id::new("b".into(), Pos::new(1, 12, 11)),
            ],
            body: Block { stmts: vec![] },
        })
    );
}

#[test]
pub fn if_statement() {
    statement_test!(
        "\
if a { }
        ";

        Statement::If(IfStatement {
            condition: Expr::Atom(Atom::Id(Id::new("a".into(), Pos::new(1, 4,  3)))),
            then_body: Block { stmts: vec![] },
            else_body: None,
        })
    );

    statement_test!(
        "\
if a { }
else { }
        ";

        Statement::If(IfStatement {
            condition: Expr::Atom(Atom::Id(Id::new("a".into(), Pos::new(1, 4,  3)))),
            then_body: Block { stmts: vec![] },
            else_body: Some(Block { stmts: vec![] }),
        })
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
            id: Id::new("name".into(), Pos::new(1, 4, 3)),
            args: vec![],
            body: Block {
                stmts: vec![Statement::Return(ReturnStatement(None))],
            },
        })
    );

    parse_test!(
        "\
fn name() {
    return a;
}
        ";

        Declaration::Function(FunctionDeclaration {
            id: Id::new("name".into(), Pos::new(1, 4,  3)),
            args: vec![],
            body: Block {
                stmts: vec![Statement::Return(ReturnStatement(Some(Expr::Atom(
                    Atom::Id(Id::new("a".into(), Pos::new(2, 12,  23))),
                ))))],
            },
        })
    );
}

#[test]
pub fn while_statement() {
    statement_test!(
        "\
while a {}
        ";

        Statement::While(WhileStatement {
            condition: Expr::Atom(Atom::Id(Id::new("a".into(), Pos::new(1, 7,  6)))),
            body: Block { stmts: vec![] },
        })
    );
}

#[test]
pub fn expr_call() {
    expr_test!(
        "call();";
        Expr::Call(Call { id: Id::new("call".into(), Pos::default()), args: vec![] })
    );

    expr_test!(
        "call(1);";
        Expr::Call(Call {
            id: Id::new("call".into(), Pos::default()),
            args: vec![
                Expr::Atom(Atom::Literal(Literal::Number(1.0)))
            ],
        })
    );

    expr_test!(
        "call(1, 2);";
        Expr::Call(Call {
            id: Id::new("call".into(), Pos::default()),
            args: vec![
                Expr::Atom(Atom::Literal(Literal::Number(1.0))),
                Expr::Atom(Atom::Literal(Literal::Number(2.0)))
            ],
        })
    );
}
