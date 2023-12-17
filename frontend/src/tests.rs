use crate::{
    ast::{
        Atom, Block, Call, DeclareStatement, Expr, ExprStatement, FunctionStatement, Id,
        IfStatement, ReturnStatement, Statement, WhileStatement,
    },
    lexer::{Literal, Pos},
    parse, Collect, Lexer, TokenStream,
};

macro_rules! parser_test {
    ($code:expr; $stmt:expr) => {
        assert_eq!(parse($code).unwrap()[0], $stmt);
    };
}

macro_rules! expr_test {
    ($code:expr; $expr:expr) => {{
        let mut token_stream =
            TokenStream::new(Lexer::new($code).collect::<Result<Vec<_>, _>>().unwrap());
        assert_eq!(Expr::collect(&mut token_stream).unwrap(), $expr);
    }};
}

#[test]
pub fn declare_statement() {
    parser_test!(
        "\
let a;
        ";
        Statement::Declare(DeclareStatement {
            id: Id::new("a".into(), Pos::new(1, 5, 0, 4)),
            init_expr: None,
        })
    );

    parser_test!(
        "\
let a = a;
        ";
        Statement::Declare(DeclareStatement {
            id: Id::new("a".into(), Pos::new(1, 5, 0, 4)),
            init_expr: Some(Expr::Atom(Atom::Id(Id::new(
                "a".into(),
                Pos::new(1, 9, 0, 8),
            )))),
        })
    );
}

#[test]
pub fn expr_statement() {
    parser_test!(
        "\
a;
        ";
        Statement::Expr(ExprStatement(Expr::Atom(Atom::Id(Id::new(
            "a".into(),
            Pos::new(1, 1, 0, 0),
        )))))
    );
}

#[test]
pub fn function_statement() {
    parser_test!(
        "\
fn name() {}
        ";
        Statement::Function(FunctionStatement {
            id: Id::new("name".into(), Pos::new(1, 4, 0, 3)),
            args: vec![],
            body: Block { stmts: vec![] },
        })
    );

    parser_test!(
        "\
fn name(a) {}
        ";
        Statement::Function(FunctionStatement {
            id: Id::new("name".into(), Pos::new(1, 4, 0, 3)),
            args: vec![
                Id::new("a".into(), Pos::new(1, 9, 0, 8))
            ],
            body: Block { stmts: vec![] },
        })
    );

    parser_test!(
        "\
fn name(a, b) {}
        ";
        Statement::Function(FunctionStatement {
            id: Id::new("name".into(), Pos::new(1, 4, 0, 3)),
            args: vec![
                Id::new("a".into(), Pos::new(1, 9, 0, 8)),
                Id::new("b".into(), Pos::new(1, 12, 0, 11)),
            ],
            body: Block { stmts: vec![] },
        })
    );
}

#[test]
pub fn if_statement() {
    parser_test!(
        "\
if a { }
        ";

        Statement::If(IfStatement {
            condition: Expr::Atom(Atom::Id(Id::new("a".into(), Pos::new(1, 4, 0, 3)))),
            then_body: Block { stmts: vec![] },
            else_body: None,
        })
    );

    parser_test!(
        "\
if a { }
else { }
        ";

        Statement::If(IfStatement {
            condition: Expr::Atom(Atom::Id(Id::new("a".into(), Pos::new(1, 4, 0, 3)))),
            then_body: Block { stmts: vec![] },
            else_body: Some(Block { stmts: vec![] }),
        })
    );
}

#[test]
pub fn return_statement() {
    parser_test!(
        "\
fn name() {
    return;
}
        ";
        Statement::Function(FunctionStatement {
            id: Id::new("name".into(), Pos::new(1, 4, 0, 3)),
            args: vec![],
            body: Block {
                stmts: vec![Statement::Return(ReturnStatement(None))],
            },
        })
    );

    parser_test!(
        "\
fn name() {
    return a;
}
        ";

        Statement::Function(FunctionStatement {
            id: Id::new("name".into(), Pos::new(1, 4, 0, 3)),
            args: vec![],
            body: Block {
                stmts: vec![Statement::Return(ReturnStatement(Some(Expr::Atom(
                    Atom::Id(Id::new("a".into(), Pos::new(2, 12, 12, 23))),
                ))))],
            },
        })
    );
}

#[test]
pub fn while_statement() {
    parser_test!(
        "\
while a {}
        ";

        Statement::While(WhileStatement {
            condition: Expr::Atom(Atom::Id(Id::new("a".into(), Pos::new(1, 7, 0, 6)))),
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
