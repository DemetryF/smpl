use derive_more::Constructor;

use crate::{
    lexer::token::token_value::TokenValue,
    parser::{
        ast::{Collect, Id},
        parser_utils::ParserUtils,
        token_stream::TokenStream,
    },
};

use super::Expr;

#[derive(Debug, Constructor)]
pub struct Call {
    pub id: Id,
    pub args: Vec<Expr>,
}

impl Collect for Call {
    fn collect(token_stream: &mut TokenStream) -> Self {
        let id = ParserUtils::id(token_stream);

        let args = Self::call_args(token_stream);

        Self::new(id, args)
    }
}

impl Call {
    fn call_args(token_stream: &mut TokenStream) -> Vec<Expr> {
        let mut args = Vec::new();

        token_stream.accept(&TokenValue::OpeningParen);

        if token_stream.skip_if(&TokenValue::ClosingParen).is_some() {
            return args;
        }

        args.push(Expr::collect(token_stream));
        while token_stream.skip_if(&TokenValue::Comma).is_some() {
            args.push(Expr::collect(token_stream));
        }

        token_stream.accept(&TokenValue::ClosingParen);

        args
    }
}
