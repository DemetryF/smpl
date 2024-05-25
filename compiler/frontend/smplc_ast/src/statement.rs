use crate::{Block, Expr, Id, Type};

#[derive(Debug, PartialEq)]
pub enum Statement<'source> {
    Declare(DeclareStatement<'source>),
    If(IfStatement<'source>),
    While(WhileStatement<'source>),
    Expr(ExprStatement<'source>),
    Return(ReturnStatement<'source>),
    Break,
    Continue,
}

#[derive(Debug, PartialEq)]
pub struct DeclareStatement<'source> {
    pub id: Id<'source>,
    pub ty: Type,
    pub value: Option<Expr<'source>>,
}

#[derive(PartialEq, Debug)]
pub enum ExprStatement<'source> {
    Expr(Expr<'source>),
    Assign { id: Id<'source>, rhs: Expr<'source> },
}

#[derive(PartialEq, Debug)]
pub struct IfStatement<'source> {
    pub cond: Expr<'source>,
    pub body: Block<'source>,
    pub else_body: Option<Block<'source>>,
}

#[derive(PartialEq, Debug)]
pub struct ReturnStatement<'source> {
    pub value: Option<Expr<'source>>,
}

#[derive(PartialEq, Debug)]
pub struct WhileStatement<'source> {
    pub cond: Expr<'source>,
    pub body: Block<'source>,
}
