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

#[derive(Debug, PartialEq)]
pub struct FunData {
    pub declared_at: Pos,
    pub id: smplc_ir::FunctionId,
    pub args_count: usize,
}

#[derive(Hash)]
pub struct VarData {
    pub declared_at: Pos,
}
