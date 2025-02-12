use std::rc::Rc;

pub use smplc_ast::{Literal, Pos, Type};

use smplc_ast::Span;

use crate::{BinOp, UnOp};

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
        fun_ref: FunRef,
        args: Vec<Self>,
    },
    Atom(Atom<'source>),
}

pub type FunRef = Rc<FunData>;
pub type VarRef = Rc<VarData>;

pub enum Atom<'source> {
    Var(VarRef),
    Literal(Literal<'source>),
}

#[derive(Debug, PartialEq)]
pub struct FunData {
    pub declared_at: Span,
    pub id: Rc<str>,
    pub ret_ty: Option<Type>,
    pub args: Vec<Type>,
}

#[derive(Hash)]
pub struct VarData {
    pub declared_at: Span,
    pub ty: Type,
}
