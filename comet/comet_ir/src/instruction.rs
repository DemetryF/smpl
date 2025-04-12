use crate::{BinOp, FunId, UnOp, Value};

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
        ty: Type,
        value: Atom,
    },
    Binary {
        dst: Id,
        op: BinOp,
        lhs: Atom,
        rhs: Atom,
    },
    Unary {
        dst: Id,
        op: UnOp,
        operand: Atom,
    },
    Call {
        dst: Option<Id>,
        fun: FunId<'f>,
        args: Vec<(Atom, Type)>,
    },
}

impl Sequental<'_> {
    pub fn dst_and_ty(&self) -> Option<(Id, Type)> {
        match self {
            &Self::Assign { dst, ty, .. } => Some((dst, ty)),
            &Self::Binary { dst, op, .. } => Some((dst, op.ty())),
            &Self::Unary { dst, op, .. } => Some((dst, op.ty())),
            &Self::Call { dst, fun, .. } => dst.zip(fun.ret_ty()),
        }
    }
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
        op: BinOp,
        rhs: Atom,
        label: Label,
    },
    Goto {
        label: Label,
    },
    Return {
        value: Option<(Type, Atom)>,
    },
    Halt,
}

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
pub enum Type {
    Real,
    Int,
    F32x2,
    F32x3,
    F32x4,
}

#[derive(Clone, Copy)]
pub enum Atom {
    Value(Value),
    Id(Id),
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct Label(pub(crate) usize);

impl Label {
    pub fn new(id: usize) -> Self {
        Label(id)
    }
}

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
pub struct Id(pub(crate) usize);

impl Id {
    pub fn new(id: usize) -> Self {
        Self(id)
    }
}
