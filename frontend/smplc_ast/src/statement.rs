use crate::{Block, Expr, Id};

#[derive(Debug, PartialEq)]
pub enum Statement {
    Declare(DeclareStatement),
    Function(FunctionStatement),
    If(IfStatement),
    While(WhileStatement),
    Expr(ExprStatement),
    Return(ReturnStatement),
}

#[derive(Debug, PartialEq)]
pub struct DeclareStatement {
    pub id: Id,
    pub init_expr: Option<Expr>,
}

#[derive(PartialEq, Debug)]
pub struct ExprStatement(pub Expr);

#[derive(PartialEq, Debug)]
pub struct FunctionStatement {
    pub id: Id,
    pub args: Vec<Id>,
    pub body: Block,
}

#[derive(PartialEq, Debug)]
pub struct IfStatement {
    pub condition: Expr,
    pub then_body: Block,
    pub else_body: Option<Block>,
}

#[derive(PartialEq, Debug)]
pub struct ReturnStatement(pub Option<Expr>);

#[derive(PartialEq, Debug)]
pub struct WhileStatement {
    pub condition: Expr,
    pub body: Block,
}
