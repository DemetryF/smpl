use std::rc::Rc;

use smplc_ast::Span;
pub use smplc_ast::{Literal, Pos, Type};

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
        fun_ref: FunRef,
        args: Vec<Self>,
    },
    Atom(Atom),
}

pub type FunRef = Rc<FunData>;
pub type VarRef = Rc<VarData>;

pub enum Atom {
    Var(VarRef),
    Literal(Literal),
}

#[derive(Debug, PartialEq)]
pub struct FunData {
    pub declared_at: Span,
    pub id: String,
    pub ret_ty: Option<Type>,
    pub args: Vec<Type>,
}

#[derive(Hash)]
pub struct VarData {
    pub declared_at: Span,
    pub ty: Type,
}
