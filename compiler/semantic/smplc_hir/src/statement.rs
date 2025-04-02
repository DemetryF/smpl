use smplc_ast::Spanned;

use crate::{Block, Expr, VarId};

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
        var: VarId,
        rhs: Spanned<Expr<'source>>,
    },
    Expr(Spanned<Expr<'source>>),
}

pub struct IfStatement<'source> {
    pub cond: Spanned<Expr<'source>>,
    pub body: Block<'source>,
    pub else_body: Option<Block<'source>>,
}

pub struct ReturnStatement<'source> {
    pub value: Option<Spanned<Expr<'source>>>,
}

pub struct WhileStatement<'source> {
    pub cond: Spanned<Expr<'source>>,
    pub body: Block<'source>,
}
