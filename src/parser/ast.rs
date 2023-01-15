use crate::lexer::token::{operator::Operator, token_value::Literal, Token};

#[derive(Debug)]
pub enum Statement<'code> {
    Expr(Expr<'code>),
    Declare {
        id: &'code str,
        expr: Option<Expr<'code>>,
    },
    Function {
        id: &'code str,
        args: Vec<&'code str>,
        body: Block<'code>,
    },
    If {
        cond: Expr<'code>,
        then_body: Block<'code>,
        else_body: Option<Block<'code>>,
    },
    Return(Option<Expr<'code>>),
    While {
        cond: Expr<'code>,
        body: Block<'code>,
    },
}

#[derive(Debug)]
pub enum Expr<'code> {
    Binary {
        left: Box<Expr<'code>>,
        op: Operator,
        right: Box<Expr<'code>>,
    },
    Unary {
        op: Operator,
        expr: Box<Expr<'code>>,
    },
    Call {
        id: &'code str,
        args: Vec<Expr<'code>>,
    },
    Atom(Atom<'code>),
}

#[derive(Debug)]
pub enum Atom<'code> {
    Literal(Literal),
    Id(&'code str),
}

#[derive(Debug)]
pub struct Block<'code>(pub Vec<Statement<'code>>);
