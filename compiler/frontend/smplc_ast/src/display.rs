use std::fmt;

use crate::{Atom, BinOp, Component, Id, Literal, UnOp};

impl fmt::Display for Atom<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Atom::Id(id) => write!(f, "{id}"),
            Atom::Literal(literal) => write!(f, "{literal}"),
        }
    }
}

impl fmt::Display for Id<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Display for BinOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Or => write!(f, "|"),
            Self::And => write!(f, "&"),
            Self::Ne => write!(f, "!="),
            Self::Eq => write!(f, "=="),
            Self::Ge => write!(f, ">="),
            Self::Gt => write!(f, ">"),
            Self::Le => write!(f, "<="),
            Self::Lt => write!(f, "<"),
            Self::Add => write!(f, "+"),
            Self::Sub => write!(f, "-"),
            Self::Mul => write!(f, "*"),
            Self::Div => write!(f, "/"),
        }
    }
}

impl fmt::Display for UnOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Not => write!(f, "!"),
            Self::Neg => write!(f, "-"),
        }
    }
}

impl fmt::Display for Literal<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl fmt::Display for Component {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Component::X => write!(f, "x"),
            Component::Y => write!(f, "y"),
            Component::Z => write!(f, "z"),
            Component::W => write!(f, "w"),
        }
    }
}
