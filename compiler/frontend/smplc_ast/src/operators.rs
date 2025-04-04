use stack_array::ArrayBuf;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum BinOp {
    Or,
    And,

    Ne,
    Eq,
    Ge,
    Gt,
    Le,
    Lt,

    Add,
    Sub,
    Mul,
    Div,
}

impl BinOp {
    pub fn power(self) -> (usize, usize) {
        match self {
            Self::Or => (3, 4),
            Self::And => (5, 6),
            Self::Ne => (7, 8),
            Self::Eq => (7, 8),
            Self::Ge => (9, 10),
            Self::Gt => (9, 10),
            Self::Le => (9, 10),
            Self::Lt => (9, 10),
            Self::Add => (11, 12),
            Self::Sub => (11, 12),
            Self::Mul => (13, 14),
            Self::Div => (13, 14),
        }
    }

    pub fn is_arithm(self) -> bool {
        use BinOp::*;

        matches!(self, Add | Sub | Mul | Div)
    }

    pub fn is_vec(self) -> bool {
        use BinOp::*;

        matches!(self, Add | Sub | Mul | Div)
    }

    pub fn is_rel(self) -> bool {
        use BinOp::*;

        matches!(self, Eq | Ne | Lt | Le | Gt | Ge)
    }

    pub fn is_logic(self) -> bool {
        use BinOp::*;

        matches!(self, And | Or)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum UnOp {
    Not,
    Neg,
}

impl UnOp {
    pub fn power(&self) -> (usize, usize) {
        match self {
            Self::Not => (0, 15),
            Self::Neg => (0, 15),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Swizzle {
    pub combination: ArrayBuf<Component, 4>,
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
