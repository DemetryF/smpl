use smplc_ast::{Spanned, Swizzle};

use crate::{FunId, VarId};

pub use smplc_ast::{BinOp, Literal, Pos, UnOp};

pub enum Expr<'source> {
    Binary {
        lhs: Box<Spanned<Self>>,
        op: BinOp,
        rhs: Box<Spanned<Self>>,
    },
    Unary {
        op: UnOp,
        rhs: Box<Spanned<Self>>,
    },
    Swizzle {
        lhs: Box<Spanned<Self>>,
        swizzle: Swizzle,
    },
    Call {
        fun: FunId,
        args: Vec<Spanned<Self>>,
    },
    Atom(Atom<'source>),
}

#[derive(Clone, Copy)]
pub enum Atom<'source> {
    Var(VarId),
    Literal(Literal<'source>),
}
