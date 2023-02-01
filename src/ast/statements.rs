use super::{Block, Expr, Id};
use derive_more::Constructor;

pub enum Statement {
    Expr(Expr),
    Declare(DeclareStatement),
    Function(FunctionStatement),
    If(IfStatement),
    Return(ReturnStatement),
    While(WhileStatement),
}

#[derive(Constructor)]
pub struct DeclareStatement {
    pub id: Id,
    pub expr: Option<Expr>,
}

pub struct FunctionStatement {
    pub id: Id,
    pub args: Vec<Id>,
    pub body: Block,

    pub has_return: bool,
}

pub struct ReturnStatement(pub Option<Expr>);

#[derive(Constructor)]
pub struct IfStatement {
    pub cond: Expr,
    pub then_body: Block,
    pub else_body: Option<Block>,
}

#[derive(Constructor)]
pub struct WhileStatement {
    pub cond: Expr,
    pub body: Block,
}
