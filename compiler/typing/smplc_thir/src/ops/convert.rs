use smplc_hir as hir;

use super::{ArithmOp, EqOp, LinearType, NumberType, OrdOp, VecType};

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

impl TryFrom<hir::BinOp> for EqOp {
    type Error = ();

    fn try_from(value: hir::BinOp) -> Result<Self, Self::Error> {
        match value {
            hir::BinOp::Ne => Ok(Self::Ne),
            hir::BinOp::Eq => Ok(Self::Eq),

            _ => Err(()),
        }
    }
}

impl TryFrom<hir::BinOp> for OrdOp {
    type Error = ();

    fn try_from(value: hir::BinOp) -> Result<Self, Self::Error> {
        match value {
            hir::BinOp::Ge => Ok(Self::Ge),
            hir::BinOp::Gt => Ok(Self::Gt),
            hir::BinOp::Le => Ok(Self::Le),
            hir::BinOp::Lt => Ok(Self::Lt),
            _ => Err(()),
        }
    }
}

impl From<NumberType> for hir::Type {
    fn from(value: NumberType) -> hir::Type {
        match value {
            NumberType::Complex => Self::Complex,
            NumberType::Real => Self::Real,
            NumberType::Int => Self::Int,
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

impl From<LinearType> for hir::Type {
    fn from(value: LinearType) -> Self {
        match value {
            LinearType::Vec(ty) => ty.into(),
            LinearType::Number(ty) => ty.into(),
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

impl TryFrom<hir::Type> for LinearType {
    type Error = ();

    fn try_from(value: hir::Type) -> Result<Self, Self::Error> {
        Result::or(
            VecType::try_from(value).map(|ty| Self::Vec(ty)),
            NumberType::try_from(value).map(|ty| Self::Number(ty)),
        )
    }
}
