mod display;
mod ops;

use std::rc::Rc;

pub use ops::*;

pub enum Instruction {
    Assign {
        res: Id,
        rhs: AssignRhs,
    },

    Goto(Label),
    IfRel {
        ty: Type,
        op: RelOp,
        lhs: Operand,
        rhs: Operand,
        label: Label,
    },

    Call(Call),
    Ret(Option<Operand>),
}

pub enum AssignRhs {
    Arithm {
        ty: Type,
        op: ArithmOp,
        lhs: Operand,
        rhs: Operand,
    },
    Neg {
        ty: Type,
        rhs: Operand,
    },
    Call(Call),
    Operand(Operand),
    Phi(Vec<(Label, Operand)>),
}

impl AssignRhs {
    pub fn ty(&self) -> Type {
        match self {
            AssignRhs::Arithm { ty, .. } => *ty,
            AssignRhs::Neg { ty, .. } => *ty,
            AssignRhs::Call(Call { id, .. }) => id.ret_ty().unwrap(),
            AssignRhs::Operand(op) => op.ty(),
            AssignRhs::Phi(branches) => branches.first().unwrap().1.ty(),
        }
    }
}

pub struct Call {
    pub id: FnId,
    pub args: Vec<Operand>,
}

pub enum Operand {
    Real(f32),
    Int(i32),
    Id(Id),
}

pub enum Value {
    Real(f32),
    Int(i32),
}

impl Operand {
    pub fn ty(&self) -> Type {
        match self {
            Self::Real(_) => Type::Real,
            Self::Int(_) => Type::Int,
            Self::Id(id) => id.ty(),
        }
    }
}

#[derive(Clone)]
pub struct Id(Rc<str>, Type);

impl Id {
    pub fn new(id: impl Into<Rc<str>>, ty: Type) -> Self {
        Self(id.into(), ty)
    }

    pub fn ty(&self) -> Type {
        self.1
    }
}

impl PartialEq for Id {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for Id {}

#[derive(Clone)]
pub struct FnId(Rc<str>, Option<Type>);

impl FnId {
    pub fn new(id: impl Into<Rc<str>>) -> Self {
        Self(id.into(), None)
    }

    pub fn with_ret_ty(id: impl Into<Rc<str>>, ret_ty: Type) -> Self {
        Self(id.into(), Some(ret_ty))
    }

    pub fn ret_ty(&self) -> Option<Type> {
        self.1
    }
}

#[derive(Clone, Copy)]
pub enum Type {
    Real,
    Int,
}

#[derive(Clone)]
pub struct Label(Rc<str>);