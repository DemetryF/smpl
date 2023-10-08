use crate::expr::{Expr, Ident};
use crate::operators::AssignOp;
use crate::Block;

#[derive(PartialEq, Debug)]
pub enum Statement<'source> {
    Break(BreakStatement),
    Continue(ContinueStatement),
    Declare(DeclareStatement<'source>),
    Expr(ExprStatement<'source>),
    Function(FunctionStatement<'source>),
    If(IfStatement<'source>),
    Return(ReturnStatement<'source>),
    While(WhileStatement<'source>),
}

#[derive(PartialEq, Debug)]
pub struct BreakStatement;

#[derive(PartialEq, Debug)]
pub struct ContinueStatement;

#[derive(PartialEq, Debug)]
pub struct DeclareStatement<'source> {
    pub id: Ident<'source>,
    pub expr: Option<Expr<'source>>,
}

#[derive(PartialEq, Debug)]
pub enum ExprStatement<'source> {
    Assign {
        lhs: Ident<'source>,
        op: AssignOp,
        rhs: Expr<'source>,
    },
    Expr(Expr<'source>),
}

#[derive(PartialEq, Debug)]
pub struct FunctionStatement<'source> {
    pub id: Ident<'source>,
    pub args: Vec<Ident<'source>>,
    pub body: Block<'source>,
}

#[derive(PartialEq, Debug)]
pub struct IfStatement<'source> {
    pub cond: Expr<'source>,
    pub then_branch: Block<'source>,
    pub else_branch: Option<Block<'source>>,
}

#[derive(PartialEq, Debug)]
pub struct ReturnStatement<'source> {
    pub expr: Option<Expr<'source>>,
}

#[derive(PartialEq, Debug)]
pub struct WhileStatement<'source> {
    pub cond: Expr<'source>,
    pub body: Block<'source>,
}
