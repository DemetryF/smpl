use crate::{Block, Expr, VarRef};

pub enum Statement<'source> {
    Expr(ExprStatement<'source>),
    If(IfStatement<'source>),
    Return(ReturnStatement<'source>),
    While(WhileStatement<'source>),
    Break,
    Continue,
}

pub enum ExprStatement<'source> {
    Assign {
        var: VarRef<'source>,
        rhs: Expr<'source>,
    },
    Expr(Expr<'source>),
}

pub struct IfStatement<'source> {
    pub cond: Expr<'source>,
    pub body: Block<'source>,
    pub else_body: Option<Block<'source>>,
}

pub struct ReturnStatement<'source> {
    pub value: Option<Expr<'source>>,
}

pub struct WhileStatement<'source> {
    pub cond: Expr<'source>,
    pub body: Block<'source>,
}
