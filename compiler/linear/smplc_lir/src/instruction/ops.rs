use std::fmt;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ArithmOp {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum RelOp {
    Eq,
    Ne,
    Gt,
    Ge,
    Lt,
    Le,
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
