use crate::lexer::token::{operator::Operator, token_value::TokenValue};

use super::{
    ast::Collect,
    ast::{expr::Expr, Id},
    token_stream::TokenStream,
};

pub struct ParserUtils;
impl ParserUtils {
    pub fn id(token_stream: &mut TokenStream) -> Id {
        let token = token_stream.skip();
        match token.value {
            TokenValue::Id(value) => Id::new(value, token.pos),
            _ => panic!("expected id"),
        }
    }

    pub fn op(token_stream: &mut TokenStream) -> Operator {
        match token_stream.skip().value {
            TokenValue::Operator(op) => op,
            _ => panic!("expected operator"),
        }
    }

    pub fn parenthesis(token_stream: &mut TokenStream) -> Expr {
        token_stream.accept(&TokenValue::OpeningParen);
        let expr = Expr::collect(token_stream);
        token_stream.accept(&TokenValue::ClosingParen);

        expr
    }
}
