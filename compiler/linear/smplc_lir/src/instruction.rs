use smplc_macros::EnumWrap;

pub use smplc_thir::{ArithmOp as BinOp, RelOp};

use crate::Number;

#[derive(EnumWrap)]
pub enum Instruction {
    Sequental(Sequental),
    ControlFlow(ControlFlow),
    Phi(Phi),
}

pub enum Sequental {
    Assign {
        dst: Id,
        value: Atom,
    },
    Binary {
        dst: Id,
        op: BinOp,
        ty: Type,
        lhs: Atom,
        rhs: Atom,
    },
    Unary {
        dst: Id,
        op: UnOp,
        ty: Type,
        operand: Atom,
    },
    Call {
        dst: Option<Id>,
        fun: FunId,
        args: Vec<(Atom, Type)>,
    },
}

pub struct Phi {
    pub dst: Id,
    pub branches: Vec<(Label, Id)>,
    pub else_value: Option<Id>,
}

#[derive(Clone, Copy)]
pub enum ControlFlow {
    If {
        lhs: Atom,
        op: RelOp,
        ty: Type,
        rhs: Atom,
        label: Label,
    },
    Goto {
        label: Label,
    },
    Return {
        value: Option<Atom>,
    },
    Halt,
}

#[derive(Clone, Copy)]
pub enum UnOp {
    Neg,
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub enum Type {
    Real,
    Int,
}

#[derive(Clone, Copy)]
pub enum Atom {
    Number(Number),
    Id(Id),
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct FunId(usize);

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct Label(usize);

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct Id(usize, Type);

impl Id {
    pub fn ty(self) -> Type {
        self.1
    }
}
