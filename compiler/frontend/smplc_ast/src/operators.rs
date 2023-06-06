use smplc_token::TokenValue;

#[derive(Clone, Copy)]
pub enum AssignOp {
    Assign,
    AddAssign,
    SubAssign,
    MulAssign,
    DivAssign,
}

impl TryFrom<TokenValue<'_>> for AssignOp {
    type Error = ();

    fn try_from(value: TokenValue) -> Result<Self, Self::Error> {
        match value {
            TokenValue::Assign => Ok(Self::Assign),
            TokenValue::AddAssign => Ok(Self::AddAssign),
            TokenValue::SubAssign => Ok(Self::SubAssign),
            TokenValue::MulAssign => Ok(Self::MulAssign),
            TokenValue::DivAssign => Ok(Self::DivAssign),

            _ => Err(()),
        }
    }
}

#[derive(Clone, Copy)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,

    Or,
    And,

    Eq,
    Ne,
    Gt,
    Ge,
    Lt,
    Le,
}

impl BinOp {
    pub fn get_bp(self) -> (u8, u8) {
        use BinOp::*;

        match self {
            Or => (3, 4),
            And => (5, 6),

            Eq | Ne => (7, 8),
            Gt | Ge | Lt | Le => (9, 10),

            Add | Sub => (11, 12),
            Mul | Div => (13, 14),
        }
    }
}

impl TryFrom<TokenValue<'_>> for BinOp {
    type Error = ();

    fn try_from(value: TokenValue) -> Result<Self, Self::Error> {
        match value {
            TokenValue::Eq => Ok(Self::Eq),
            TokenValue::Ne => Ok(Self::Ne),
            TokenValue::Ge => Ok(Self::Ge),
            TokenValue::Gt => Ok(Self::Gt),
            TokenValue::Le => Ok(Self::Le),
            TokenValue::Lt => Ok(Self::Lt),

            TokenValue::Or => Ok(Self::Or),
            TokenValue::And => Ok(Self::And),

            TokenValue::Plus => Ok(Self::Add),
            TokenValue::Minus => Ok(Self::Sub),
            TokenValue::Asterisk => Ok(Self::Mul),
            TokenValue::Slash => Ok(Self::Div),

            _ => Err(()),
        }
    }
}

#[derive(Clone, Copy)]
pub enum UnOp {
    Neg,
    Not,
}

impl UnOp {
    pub fn get_bp(self) -> u8 {
        match self {
            UnOp::Neg | UnOp::Not => 15,
        }
    }
}

impl TryFrom<TokenValue<'_>> for UnOp {
    type Error = ();

    fn try_from(value: TokenValue) -> Result<Self, Self::Error> {
        match value {
            TokenValue::Minus => Ok(Self::Neg),
            TokenValue::Not => Ok(Self::Not),

            _ => Err(()),
        }
    }
}
