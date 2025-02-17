use std::rc::Rc;

use smplc_ast as ast;

pub use smplc_ast::{BinOp, Literal, Pos, Type, UnOp};

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
        fun_ref: FunRef<'source>,
        args: Vec<Self>,
    },
    Atom(Atom<'source>),
}

pub type FunRef<'source> = Rc<FunData<'source>>;
pub type VarRef<'source> = Rc<VarData<'source>>;

pub enum Atom<'source> {
    Var(VarRef<'source>),
    Literal(Literal<'source>),
}

#[derive(Debug, PartialEq, Eq)]
pub struct FunData<'source> {
    pub id: ast::Id<'source>,
    pub ret_ty: Option<Type>,
    pub args_types: Vec<Type>,
}

#[derive(Hash)]
pub struct VarData<'source> {
    pub id: ast::Id<'source>,
    pub ty: Option<Type>,
}
