use smplc_token::{Literal, Posed};

use crate::operators::{AssignOp, BinOp, UnOp};

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

pub struct BreakStatement;
pub struct ContinueStatement;

pub struct DeclareStatement<'source> {
    pub id: Ident<'source>,
    pub expr: Option<Expr<'source>>,
}

pub enum ExprStatement<'source> {
    Assign {
        lhs: Ident<'source>,
        op: AssignOp,
        rhs: Expr<'source>,
    },
    Expr(Expr<'source>),
}

pub struct FunctionStatement<'source> {
    pub id: Ident<'source>,
    pub args: Vec<Ident<'source>>,
    pub body: Block<'source>,
}

pub struct IfStatement<'source> {
    pub cond: Expr<'source>,
    pub then_branch: Block<'source>,
    pub else_branch: Option<Block<'source>>,
}

pub struct ReturnStatement<'source> {
    pub expr: Option<Expr<'source>>,
}

pub struct WhileStatement<'source> {
    pub cond: Expr<'source>,
    pub body: Block<'source>,
}

pub struct Block<'source> {
    pub statements: Vec<Statement<'source>>,
}

pub enum Expr<'source> {
    Binary {
        lhs: Box<Self>,
        op: BinOp,
        rhs: Box<Self>,
    },
    Unary {
        op: UnOp,
        rhs: Box<Self>,
    },
    Call {
        id: Ident<'source>,
        args: Vec<Self>,
    },
    Atom(Atom<'source>),
}

pub enum Atom<'source> {
    Ident(Ident<'source>),
    Literal(Literal),
}

pub type Ident<'source> = Posed<&'source str>;
