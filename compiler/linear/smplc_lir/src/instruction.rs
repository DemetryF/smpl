use smplc_macros::EnumWrap;
use smplc_thir as thir;
use smplc_thir::FunId;

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

#[derive(Debug)]
pub struct Phi {
    pub dst: Id,
    pub branches: Vec<Id>,
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

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
pub enum Type {
    Real,
    Int,
}

impl From<thir::Type> for Type {
    fn from(value: thir::Type) -> Self {
        match value {
            thir::Type::Real => Type::Real,
            thir::Type::Int | thir::Type::Bool => Type::Int,
        }
    }
}

impl From<thir::NumberType> for Type {
    fn from(value: thir::NumberType) -> Self {
        match value {
            thir::NumberType::Real => Type::Real,
            thir::NumberType::Int => Type::Int,
        }
    }
}

#[derive(Clone, Copy)]
pub enum Atom {
    Number(Number),
    Id(Id),
}

impl Atom {
    pub fn ty(self) -> Type {
        match self {
            Atom::Number(Number::Real(_)) => Type::Real,
            Atom::Number(Number::Int(_)) => Type::Int,
            Atom::Id(id) => id.ty(),
        }
    }
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct Label(pub(crate) usize);

impl Label {
    pub fn new(id: usize) -> Self {
        Label(id)
    }
}

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
pub struct Id(pub(crate) usize, Type);

impl Id {
    pub fn new(id: usize, ty: Type) -> Self {
        Self(id, ty)
    }

    pub fn ty(self) -> Type {
        self.1
    }
}
