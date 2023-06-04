use smplc_token::TokenValue;

pub trait Op: TryFrom<TokenValue> {
    type Power;

    fn get_bp(self) -> Self::Power;
}

pub enum AssignOp {
    Assign,
    AddAssign,
    SubAssign,
    MulAssign,
    DivAssign,
}

impl Op for AssignOp {
    type Power = (u8, u8);

    fn get_bp(self) -> Self::Power {
        (2, 1)
    }
}

impl TryFrom<TokenValue> for AssignOp {
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

impl Op for BinOp {
    type Power = (u8, u8);

    fn get_bp(self) -> Self::Power {
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

impl TryFrom<TokenValue> for BinOp {
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

pub enum UnOp {
    Neg,
    Not,
}

impl Op for UnOp {
    type Power = ((), u8);

    fn get_bp(self) -> Self::Power {
        use UnOp::*;

        match self {
            Neg | Not => ((), 15),
        }
    }
}

impl TryFrom<TokenValue> for UnOp {
    type Error = ();

    fn try_from(value: TokenValue) -> Result<Self, Self::Error> {
        match value {
            TokenValue::Minus => Ok(Self::Neg),
            TokenValue::Not => Ok(Self::Not),

            _ => Err(())
        }
    }
}
