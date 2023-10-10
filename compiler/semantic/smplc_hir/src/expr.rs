pub use smplc_ast::Literal;

use crate::operators::{BinOp, UnOp};
use crate::{FunInfo, VarInfo};

pub enum Expr {
    Binary {
        lhs: Box<Self>,
        op: BinOp,
        rhs: Box<Self>,
    },
    Unary {
        op: UnOp,
    },
    Call {
        id: FunInfo,
        args: Vec<VarInfo>,
    },
    Atom(Atom),
}

pub enum Atom {
    Ident(VarInfo),
    Literal(Literal),
}
