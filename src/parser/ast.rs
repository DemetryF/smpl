use derive_more::Constructor;

use crate::lexer::token::{operator::Operator, token_value::Literal};

#[derive(Debug)]
pub enum Statement<'code> {
    Expr(Expr<'code>),
    Declare(DeclareStatement<'code>),
    Function(FunctionStatement<'code>),
    If(IfStatement<'code>),
    Return(ReturnStatement<'code>),
    While(WhileStatement<'code>),
}

#[derive(Debug, Constructor)]
pub struct DeclareStatement<'code> {
    pub id: &'code str,
    pub expr: Option<Expr<'code>>,
}

#[derive(Debug, Constructor)]
pub struct FunctionStatement<'code> {
    pub id: &'code str,
    pub args: Vec<&'code str>,
    pub body: Block<'code>,
}

#[derive(Debug, Constructor)]
pub struct IfStatement<'code> {
    pub cond: Expr<'code>,
    pub then_body: Block<'code>,
    pub else_body: Option<Block<'code>>,
}

#[derive(Debug, Constructor)]
pub struct WhileStatement<'code> {
    pub cond: Expr<'code>,
    pub body: Block<'code>,
}

#[derive(Debug, Constructor)]
pub struct ReturnStatement<'code>(Option<Expr<'code>>);

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

#[derive(Debug, Constructor)]
pub struct Block<'code>(pub Vec<Statement<'code>>);
