use std::rc::Rc;

use smplc_ast::Pos;

use crate::{BinOp, UnOp};

pub enum Expr {
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
        function: FunRef,
        args: Vec<Self>,
    },
    Atom(Atom),
}

pub type FunRef = Rc<FunData>;
pub type VarRef = Rc<VarData>;

pub enum Atom {
    Var(VarRef),
    Value(f32),
}

pub struct FunData {
    pub declared_at: Pos,
    pub name: String,
    pub args_count: usize,
}

pub struct VarData {
    pub declared_at: Pos,
    pub name: String,
}
