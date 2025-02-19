pub use smplc_ast::{BinOp, Literal, Pos, Type, UnOp};

use crate::{FunId, VarId};

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
        fun: FunId,
        args: Vec<Self>,
    },
    Atom(Atom<'source>),
}

pub enum Atom<'source> {
    Var(VarId),
    Literal(Literal<'source>),
}
