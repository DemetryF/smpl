use std::fmt;

use crate::{ArithmOp, NumberType, RelOp};

impl fmt::Display for RelOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RelOp::Eq => write!(f, "eq"),
            RelOp::Ne => write!(f, "ne"),
            RelOp::Gt => write!(f, "gt"),
            RelOp::Ge => write!(f, "ge"),
            RelOp::Lt => write!(f, "lt"),
            RelOp::Le => write!(f, "le"),
        }
    }
}

impl fmt::Display for ArithmOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ArithmOp::Add => write!(f, "add"),
            ArithmOp::Sub => write!(f, "sub"),
            ArithmOp::Mul => write!(f, "mul"),
            ArithmOp::Div => write!(f, "div"),
        }
    }
}

impl fmt::Display for NumberType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NumberType::Real => write!(f, "real"),
            NumberType::Int => write!(f, "int"),
        }
    }
}
