use comet_ir as lir;
use smplc_hir::{self as hir, Type};

use super::{ArithmOp, BinOp, EqOp, LinearType, NumberType, OrdOp, VecOp, VecType};

impl From<BinOp> for lir::BinOp {
    fn from(value: BinOp) -> Self {
        match value {
            BinOp::Arithm(op, NumberType::Int) => lir::BinOp::Int(op.into()),
            BinOp::Arithm(op, NumberType::Real) => lir::BinOp::Real(op.into()),

            BinOp::Arithm(ArithmOp::Mul, NumberType::Complex) => lir::BinOp::ComplexMul,
            BinOp::Arithm(ArithmOp::Div, NumberType::Complex) => lir::BinOp::ComplexDiv,

            BinOp::Arithm(ArithmOp::Add, NumberType::Complex) => {
                lir::BinOp::F32s(lir::Dims::X2, lir::F32sOp::Add)
            }

            BinOp::Arithm(ArithmOp::Sub, NumberType::Complex) => {
                lir::BinOp::F32s(lir::Dims::X2, lir::F32sOp::Sub)
            }

            BinOp::Vec(op, ty) => {
                let op = match op {
                    VecOp::Add => lir::F32sOp::Add,
                    VecOp::Sub => lir::F32sOp::Sub,
                    VecOp::LeftMul | VecOp::RightMul => lir::F32sOp::ScalarMul,
                    VecOp::Div => lir::F32sOp::ScalarDiv,
                };

                lir::BinOp::F32s(ty.dims(), op)
            }

            BinOp::Eq(op, LinearType::Number(NumberType::Complex)) => match op {
                EqOp::Eq => lir::BinOp::F32s(lir::Dims::X2, lir::F32sOp::Eq),
                EqOp::Ne => lir::BinOp::F32s(lir::Dims::X2, lir::F32sOp::Ne),
            },

            BinOp::Eq(op, LinearType::Number(ty)) => {
                let op = match op {
                    EqOp::Eq => lir::ArithmOp::Eq,
                    EqOp::Ne => lir::ArithmOp::Ne,
                };

                match ty {
                    NumberType::Real => lir::BinOp::Real(op),
                    NumberType::Int => lir::BinOp::Int(op),
                    NumberType::Complex => unreachable!(),
                }
            }

            BinOp::Eq(op, LinearType::Vec(ty)) => match op {
                EqOp::Eq => lir::BinOp::F32s(ty.dims(), lir::F32sOp::Eq),
                EqOp::Ne => lir::BinOp::F32s(ty.dims(), lir::F32sOp::Ne),
            },

            BinOp::Ord(op, ty) => {
                let op = match op {
                    OrdOp::Gt => lir::ArithmOp::Gt,
                    OrdOp::Ge => lir::ArithmOp::Ge,
                    OrdOp::Lt => lir::ArithmOp::Lt,
                    OrdOp::Le => lir::ArithmOp::Le,
                };

                match ty {
                    NumberType::Real => lir::BinOp::Real(op),
                    NumberType::Int => lir::BinOp::Int(op),
                    NumberType::Complex => unreachable!(),
                }
            }

            BinOp::Or | BinOp::And => unreachable!(),
        }
    }
}

impl Into<lir::Type> for LinearType {
    fn into(self) -> lir::Type {
        Type::from(self).into()
    }
}

impl Into<lir::ArithmOp> for ArithmOp {
    fn into(self) -> lir::ArithmOp {
        match self {
            ArithmOp::Add => lir::ArithmOp::Add,
            ArithmOp::Sub => lir::ArithmOp::Sub,
            ArithmOp::Mul => lir::ArithmOp::Mul,
            ArithmOp::Div => lir::ArithmOp::Div,
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
