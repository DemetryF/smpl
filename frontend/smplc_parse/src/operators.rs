use smplc_ast::{BinOp, UnOp};
use smplc_lexer::TokenValue;

use crate::{TokenStream, TryParse};

impl TryParse for BinOp {
    fn try_parse(token_stream: &mut TokenStream) -> Option<Self> {
        let op = match token_stream.current().value {
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
