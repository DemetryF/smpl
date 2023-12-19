use smplc_lexer::{Token, TokenValue};

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

impl TryFrom<&Token> for BinOp {
    type Error = ();
    fn try_from(token: &Token) -> Result<Self, Self::Error> {
        let op = match token.value {
            TokenValue::Assignment => Self::Assignment,
            TokenValue::Or => Self::Or,
            TokenValue::And => Self::And,
            TokenValue::NotEqual => Self::NotEqual,
            TokenValue::Equal => Self::Equal,
            TokenValue::GreaterOrEqual => Self::GreaterOrEqual,
            TokenValue::Greater => Self::Greater,
            TokenValue::LessOrEqual => Self::LessOrEqual,
            TokenValue::Less => Self::Less,
            TokenValue::Plus => Self::Addition,
            TokenValue::Minus => Self::Subtraction,
            TokenValue::Star => Self::Multiplication,
            TokenValue::Slash => Self::Division,

            _ => return Err(()),
        };

        Ok(op)
    }
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

impl TryFrom<&Token> for UnOp {
    type Error = ();
    fn try_from(token: &Token) -> Result<Self, Self::Error> {
        let op = match token.value {
            TokenValue::Not => Self::Not,
            TokenValue::Minus => Self::Neg,
            _ => return Err(()),
        };
        Ok(op)
    }
}

impl UnOp {
    pub fn power(&self) -> (usize, usize) {
        match self {
            Self::Not => (0, 15),
            Self::Neg => (0, 15),
        }
    }
}
