use crate::{Block, Expr, Id};

#[derive(Debug, PartialEq)]
pub enum Statement<'source> {
    Declare(DeclareStatement<'source>),
    Function(FunctionStatement<'source>),
    If(IfStatement<'source>),
    While(WhileStatement<'source>),
    Expr(ExprStatement<'source>),
    Return(ReturnStatement<'source>),
}

#[derive(Debug, PartialEq)]
pub struct DeclareStatement<'source> {
    pub id: Id<'source>,
    pub init_expr: Option<Expr<'source>>,
}

#[derive(PartialEq, Debug)]
pub struct ExprStatement<'source>(pub Expr<'source>);

#[derive(PartialEq, Debug)]
pub struct FunctionStatement<'source> {
    pub id: Id<'source>,
    pub args: Vec<Id<'source>>,
    pub body: Block<'source>,
}

#[derive(PartialEq, Debug)]
pub struct IfStatement<'source> {
    pub condition: Expr<'source>,
    pub then_body: Block<'source>,
    pub else_body: Option<Block<'source>>,
}

#[derive(PartialEq, Debug)]
pub struct ReturnStatement<'source>(pub Option<Expr<'source>>);

#[derive(PartialEq, Debug)]
pub struct WhileStatement<'source> {
    pub condition: Expr<'source>,
    pub body: Block<'source>,
}
