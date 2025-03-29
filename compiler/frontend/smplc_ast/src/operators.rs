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
