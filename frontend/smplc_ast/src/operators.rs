#[derive(Debug, PartialEq, Clone)]
pub enum BinOp {
    Assign,

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
    pub fn power(&self) -> (usize, usize) {
        match self {
            Self::Assign => (2, 1),
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
