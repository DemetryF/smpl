use crate::Type;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum BinOp {
    Int(ArithmOp),
    Real(ArithmOp),
    F32s(Dims, F32sOp),
    IntRel(RelOp),
    RealRel(RelOp),
    F32sRel(Dims, EqOp),
    ComplexMul,
    ComplexDiv,
}

impl BinOp {
    pub fn ty(self) -> Type {
        match self {
            Self::Int(..) => Type::Int,
            Self::Real(..) => Type::Real,
            Self::F32s(dims, ..) => dims.ty(),

            Self::ComplexMul | Self::ComplexDiv => Type::F32x2,

            Self::IntRel(..) => Type::Int,
            Self::RealRel(..) => Type::Int,
            Self::F32sRel(..) => Type::Int,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ArithmOp {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum F32sOp {
    Add,
    Sub,
    ScalarMul,
    ScalarDiv,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum RelOp {
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum EqOp {
    Eq,
    Ne,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Dims {
    X2 = 2,
    X3 = 3,
    X4 = 4,
}

impl Dims {
    pub fn ty(self) -> Type {
        match self {
            Dims::X2 => Type::F32x2,
            Dims::X3 => Type::F32x3,
            Dims::X4 => Type::F32x4,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum UnOp {
    Neg(Type),
    Swizzle(Swizzle),
}

impl UnOp {
    pub fn ty(self) -> Type {
        match self {
            Self::Neg(ty) => ty,
            Self::Swizzle(swizzle) => swizzle.ty(),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
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

    pub fn ty(self) -> Type {
        match self {
            Swizzle::X1(..) => Type::Real,
            Swizzle::X2(..) => Type::F32x2,
            Swizzle::X3(..) => Type::F32x3,
            Swizzle::X4(..) => Type::F32x4,
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

impl TryFrom<char> for Component {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'x' | 'X' => Ok(Self::X),
            'y' | 'Y' => Ok(Self::Y),
            'z' | 'Z' => Ok(Self::Z),
            'w' | 'W' => Ok(Self::W),

            _ => Err(()),
        }
    }
}
