use crate::{Block, VarRef};

use crate::expr::Expr;
use crate::operators::AssignOp;

pub enum Statement {
    Break(BreakStatement),
    Continue(ContinueStatement),
    Expr(ExprStatement),
    If(IfStatement),
    Return(ReturnStatement),
    While(WhileStatement),
}

pub struct BreakStatement;
pub struct ContinueStatement;

pub enum ExprStatement {
    Assign {
        lhs: VarRef,
        op: AssignOp,
        rhs: Expr,
    },
    Expr(Expr),
}
pub struct IfStatement {
    pub cond: Expr,
    pub then_branch: Block,
    pub else_branch: Option<Block>,
}

pub struct ReturnStatement {
    pub expr: Expr,
}

pub struct WhileStatement {
    pub cond: Expr,
    pub body: Block,
}
