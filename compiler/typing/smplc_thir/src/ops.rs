use comet_ir as lir;
use comet_ir::Dims;

mod convert;
mod display;

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum BinOp {
    Arithm(ArithmOp, NumberType),
    Vec(VecOp, VecType),
    Ord(OrdOp, NumberType),
    Eq(EqOp, LinearType),
    Or,
    And,
}

impl BinOp {
    pub fn ty(self) -> lir::Type {
        match self {
            BinOp::Arithm(_, NumberType::Int) => lir::Type::Int,
            BinOp::Arithm(_, NumberType::Real) => lir::Type::Real,
            BinOp::Arithm(_, NumberType::Complex) => lir::Type::F32x2,

            BinOp::Vec(_, VecType::Vec2) => lir::Type::F32x2,
            BinOp::Vec(_, VecType::Vec3) => lir::Type::F32x4,
            BinOp::Vec(_, VecType::Vec4) => lir::Type::F32x3,

            BinOp::Ord(_, _) | BinOp::Eq(_, _) | BinOp::Or | BinOp::And => lir::Type::Int,
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum UnOp {
    Neg(LinearType),
    Not,
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum RelOp {
    Ord(OrdOp, NumberType),
    Eq(EqOp, LinearType),
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ArithmOp {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum NumberType {
    Complex,
    Real,
    Int,
}

#[derive(PartialEq, Eq, Clone, Copy)]
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
pub enum VecType {
    Vec2,
    Vec3,
    Vec4,
}

impl VecType {
    pub fn dims(self) -> Dims {
        match self {
            VecType::Vec2 => Dims::X2,
            VecType::Vec3 => Dims::X3,
            VecType::Vec4 => Dims::X4,
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum EqOp {
    Eq,
    Ne,
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum LinearType {
    Vec(VecType),
    Number(NumberType),
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum OrdOp {
    Gt,
    Ge,
    Lt,
    Le,
}
