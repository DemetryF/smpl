#[derive(Debug, PartialEq, Clone)]
pub enum BinOp {
    Assignment,
    Or,
    And,
    NotEqual,
    Equal,
    GreaterOrEqual,
    Greater,
    LessOrEqual,
    Less,
    Addition,
    Subtraction,
    Multiplication,
    Division,
}

impl BinOp {
    pub fn power(&self) -> (usize, usize) {
        match self {
            Self::Assignment => (2, 1),
            Self::Or => (3, 4),
            Self::And => (5, 6),
            Self::NotEqual => (7, 8),
            Self::Equal => (7, 8),
            Self::GreaterOrEqual => (9, 10),
            Self::Greater => (9, 10),
            Self::LessOrEqual => (9, 10),
            Self::Less => (9, 10),
            Self::Addition => (11, 12),
            Self::Subtraction => (11, 12),
            Self::Multiplication => (13, 14),
            Self::Division => (13, 14),
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
