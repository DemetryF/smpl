use smplc_ast::{BinOp, UnOp};
use smplc_lexer::TokenTag;

use crate::{token_stream::Tokens, TokenStream, TryParse};

impl<'source> TryParse<'source> for BinOp {
    fn try_parse<TS: Tokens<'source>>(token_stream: &mut TokenStream<'source, TS>) -> Option<Self> {
        let op = match token_stream.current().tag {
            TokenTag::Or => Self::Or,
            TokenTag::And => Self::And,
            TokenTag::Ne => Self::Ne,
            TokenTag::Eq => Self::Eq,
            TokenTag::Ge => Self::Ge,
            TokenTag::Gt => Self::Gt,
            TokenTag::Le => Self::Le,
            TokenTag::Lt => Self::Lt,
            TokenTag::Plus => Self::Add,
            TokenTag::Minus => Self::Sub,
            TokenTag::Star => Self::Mul,
            TokenTag::Slash => Self::Div,

            _ => return None,
        };

        Some(op)
    }
}

impl<'source> TryParse<'source> for UnOp {
    fn try_parse<TS: Tokens<'source>>(token_stream: &mut TokenStream<'source, TS>) -> Option<Self> {
        let op = match token_stream.current().tag {
            TokenTag::Not => Self::Not,
            TokenTag::Minus => Self::Neg,

            _ => return None,
        };

        Some(op)
    }
}
