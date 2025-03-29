use std::fmt;

use smplc_hir as hir;

#[derive(PartialEq, Eq)]
pub enum BinOp {
    Arithm(ArithmOp, NumberType),
    Vec(VecOp, VecType),
    Rel(RelOp, NumberType),
    Or,
    And,
}

#[derive(PartialEq, Eq)]
pub enum ArithmOp {
    Add,
    Sub,
    Mul,
    Div,
}

impl fmt::Display for ArithmOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ArithmOp::Add => write!(f, "+"),
            ArithmOp::Sub => write!(f, "-"),
            ArithmOp::Mul => write!(f, "*"),
            ArithmOp::Div => write!(f, "/"),
        }
    }
}

impl TryFrom<hir::BinOp> for ArithmOp {
    type Error = ();

    fn try_from(value: hir::BinOp) -> Result<Self, Self::Error> {
        match value {
            hir::BinOp::Add => Ok(Self::Add),
            hir::BinOp::Sub => Ok(Self::Sub),
            hir::BinOp::Mul => Ok(Self::Mul),
            hir::BinOp::Div => Ok(Self::Div),
            _ => Err(()),
        }
    }
}

#[derive(PartialEq, Eq)]
pub enum VecOp {
    Add,
    Sub,
    /// Scalar x Vec
    LeftMul,
    /// Vec x Scalar
    RightMul,
    Div,
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum RelOp {
    Eq,
    Ne,
    Gt,
    Ge,
    Lt,
    Le,
}

impl fmt::Display for RelOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RelOp::Eq => write!(f, "=="),
            RelOp::Ne => write!(f, "!="),
            RelOp::Gt => write!(f, ">"),
            RelOp::Ge => write!(f, ">="),
            RelOp::Lt => write!(f, "<"),
            RelOp::Le => write!(f, "<="),
        }
    }
}

impl TryFrom<hir::BinOp> for RelOp {
    type Error = ();

    fn try_from(value: hir::BinOp) -> Result<Self, Self::Error> {
        match value {
            hir::BinOp::Ne => Ok(Self::Ne),
            hir::BinOp::Eq => Ok(Self::Eq),
            hir::BinOp::Ge => Ok(Self::Ge),
            hir::BinOp::Gt => Ok(Self::Gt),
            hir::BinOp::Le => Ok(Self::Le),
            hir::BinOp::Lt => Ok(Self::Lt),
            _ => Err(()),
        }
    }
}

#[derive(PartialEq, Eq)]
pub enum UnOp {
    Neg(LinearType),
    Not,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum NumberType {
    Complex,
    Real,
    Int,
}

impl Into<hir::Type> for NumberType {
    fn into(self) -> hir::Type {
        match self {
            Self::Complex => hir::Type::Complex,
            Self::Real => hir::Type::Real,
            Self::Int => hir::Type::Int,
        }
    }
}

impl TryFrom<hir::Type> for NumberType {
    type Error = ();

    fn try_from(value: hir::Type) -> Result<NumberType, Self::Error> {
        match value {
            hir::Type::Complex => Ok(NumberType::Complex),
            hir::Type::Real => Ok(NumberType::Real),
            hir::Type::Int => Ok(NumberType::Int),

            _ => Err(()),
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum VecType {
    Vec2,
    Vec3,
    Vec4,
}

impl TryFrom<hir::Type> for VecType {
    type Error = ();

    fn try_from(value: hir::Type) -> Result<Self, Self::Error> {
        match value {
            hir::Type::Vec2 => Ok(Self::Vec2),
            hir::Type::Vec3 => Ok(Self::Vec3),
            hir::Type::Vec4 => Ok(Self::Vec4),

            _ => Err(()),
        }
    }
}

impl From<VecType> for hir::Type {
    fn from(value: VecType) -> Self {
        match value {
            VecType::Vec2 => Self::Vec2,
            VecType::Vec3 => Self::Vec3,
            VecType::Vec4 => Self::Vec4,
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum LinearType {
    Vec(VecType),
    Number(NumberType),
}

impl TryFrom<hir::Type> for LinearType {
    type Error = ();

    fn try_from(value: hir::Type) -> Result<Self, Self::Error> {
        VecType::try_from(value)
            .map(|ty| Self::Vec(ty))
            .or(NumberType::try_from(value).map(|ty| Self::Number(ty)))
    }
}

impl From<LinearType> for hir::Type {
    fn from(value: LinearType) -> Self {
        match value {
            LinearType::Vec(ty) => ty.into(),
            LinearType::Number(ty) => ty.into(),
        }
    }
}
