use std::fmt;

use smplc_hir as hir;

use super::{ArithmOp, EqOp, LinearType, NumberType, OrdOp, RelOp};

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

impl fmt::Display for EqOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Eq => write!(f, "=="),
            Self::Ne => write!(f, "!="),
        }
    }
}

impl fmt::Display for OrdOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OrdOp::Gt => write!(f, ">"),
            OrdOp::Ge => write!(f, ">="),
            OrdOp::Lt => write!(f, "<"),
            OrdOp::Le => write!(f, "<="),
        }
    }
}

impl fmt::Display for RelOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RelOp::Ord(op, ty) => write!(f, "{ty}.{op}"),
            RelOp::Eq(op, ty) => write!(f, "{ty}.{op}"),
        }
    }
}

impl fmt::Display for NumberType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", hir::Type::from(*self))
    }
}

impl fmt::Display for LinearType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", hir::Type::from(*self))
    }
}
