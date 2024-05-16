use crate::{Block, Expr, VarRef};

pub enum Statement {
    Expr(ExprStatement),
    If(IfStatement),
    Return(ReturnStatement),
    While(WhileStatement),
    Break,
    Continue,
}

pub enum ExprStatement {
    Assign { var: VarRef, rhs: Expr },
    Expr(Expr),
}

pub struct IfStatement {
    pub cond: Expr,
    pub body: Block,
    pub else_body: Option<Block>,
}

pub struct ReturnStatement {
    pub value: Option<Expr>,
}

pub struct WhileStatement {
    pub cond: Expr,
    pub body: Block,
}
