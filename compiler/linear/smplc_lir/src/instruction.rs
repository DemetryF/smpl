use smplc_ast::Swizzle;
use smplc_thir as thir;

pub use smplc_thir::{ArithmOp as BinOp, EqOp, LinearType, NumberType, OrdOp, RelOp, VecType};

use crate::{FunId, Value};

pub enum Instruction<'f> {
    Sequental(Sequental<'f>),
    ControlFlow(ControlFlow),
    Phi(Phi),
}

impl<'f> From<Sequental<'f>> for Instruction<'f> {
    fn from(value: Sequental<'f>) -> Self {
        Self::Sequental(value)
    }
}

impl From<ControlFlow> for Instruction<'_> {
    fn from(value: ControlFlow) -> Self {
        Self::ControlFlow(value)
    }
}

impl From<Phi> for Instruction<'_> {
    fn from(value: Phi) -> Self {
        Self::Phi(value)
    }
}

pub enum Sequental<'f> {
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
        fun: FunId<'f>,
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

pub enum UnOp {
    Neg,
    Swizzle(Swizzle),
}

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
pub enum Type {
    Complex,
    Real,
    Int,
    Vec2,
    Vec3,
    Vec4,
}

impl From<thir::Type> for Type {
    fn from(value: thir::Type) -> Self {
        match value {
            thir::Type::Real => Self::Real,
            thir::Type::Int => Self::Int,
            thir::Type::Bool => Self::Int,
            thir::Type::Vec2 => Self::Vec2,
            thir::Type::Vec3 => Self::Vec3,
            thir::Type::Vec4 => Self::Vec4,
            thir::Type::Complex => Self::Complex,
        }
    }
}

impl From<thir::NumberType> for Type {
    fn from(value: thir::NumberType) -> Self {
        match value {
            thir::NumberType::Real => Type::Real,
            thir::NumberType::Int => Type::Int,
            thir::NumberType::Complex => Type::Complex,
        }
    }
}

impl From<thir::VecType> for Type {
    fn from(value: thir::VecType) -> Self {
        match value {
            thir::VecType::Vec2 => Self::Vec2,
            thir::VecType::Vec3 => Self::Vec3,
            thir::VecType::Vec4 => Self::Vec4,
        }
    }
}

impl From<thir::LinearType> for Type {
    fn from(value: thir::LinearType) -> Self {
        match value {
            thir::LinearType::Vec(ty) => ty.into(),
            thir::LinearType::Number(ty) => ty.into(),
        }
    }
}

#[derive(Clone, Copy)]
pub enum Atom {
    Value(Value),
    Id(Id),
}

impl Atom {
    pub fn ty(self) -> Type {
        match self {
            Atom::Value(value) => value.ty(),
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
