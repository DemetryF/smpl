use crate::Type;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum BinOp {
    Int(ArithmOp),
    Real(ArithmOp),
    F32s(Dims, F32sOp),
    ComplexMul,
    ComplexDiv,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ArithmOp {
    Add,
    Sub,
    Mul,
    Div,
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum F32sOp {
    Add,
    Sub,
    ScalarMul,
    ScalarDiv,
    Eq,
    Ne,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Dims {
    X2 = 2,
    X3 = 3,
    X4 = 4,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum UnOp {
    Neg(Type),
    Swizzle(Swizzle),
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Swizzle {
    X1([Component; 1]),
    X2([Component; 2]),
    X3([Component; 3]),
    X4([Component; 4]),
}

impl Swizzle {
    pub fn as_slice(&self) -> &[Component] {
        match self {
            Swizzle::X1(comb) => &comb[..],
            Swizzle::X2(comb) => &comb[..],
            Swizzle::X3(comb) => &comb[..],
            Swizzle::X4(comb) => &comb[..],
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy, Eq, PartialOrd, Ord)]
pub enum Component {
    X,
    Y,
    Z,
    W,
}
