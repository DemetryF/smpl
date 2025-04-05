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

#[derive(PartialEq, Eq, Clone, Copy)]
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
