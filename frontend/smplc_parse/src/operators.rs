use smplc_ast::{BinOp, UnOp};
use smplc_lexer::TokenValue;

use crate::{TokenStream, TryParse};

impl TryParse for BinOp {
    fn try_parse(token_stream: &mut TokenStream) -> Option<Self> {
        let op = match token_stream.current().value {
            TokenValue::Assign => Self::Assign,
            TokenValue::Or => Self::Or,
            TokenValue::And => Self::And,
            TokenValue::Ne => Self::Ne,
            TokenValue::Eq => Self::Eq,
            TokenValue::Ge => Self::Ge,
            TokenValue::Gt => Self::Gt,
            TokenValue::Le => Self::Le,
            TokenValue::Lt => Self::Lt,
            TokenValue::Plus => Self::Add,
            TokenValue::Minus => Self::Sub,
            TokenValue::Star => Self::Mul,
            TokenValue::Slash => Self::Div,

            _ => return None,
        };

        Some(op)
    }
}

impl TryParse for UnOp {
    fn try_parse(token_stream: &mut TokenStream) -> Option<Self> {
        let op = match token_stream.current().value {
            TokenValue::Not => Self::Not,
            TokenValue::Minus => Self::Neg,

            _ => return None,
        };

        Some(op)
    }
}