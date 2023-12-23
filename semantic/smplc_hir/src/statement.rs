use crate::{Block, Expr, VarRef};

pub enum Statement {
    Expr(ExprStatement),
    If(IfStatement),
    Return(ReturnStatement),
    While(WhileStatement),
}

pub enum ExprStatement {
    Assign { to: VarRef, what: Expr },
    Expr(Expr),
}

pub struct IfStatement {
    pub cond: Expr,
    pub then_body: Block,
    pub else_body: Option<Block>,
}

pub struct ReturnStatement {
    pub expr: Option<Expr>,
}

pub struct WhileStatement {
    pub cond: Expr,
    pub body: Block,
}
