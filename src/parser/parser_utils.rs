use crate::{
    error::Error,
    lexer::token::{operator::Operator, token_value::TokenValue},
};

use super::{ast::expr::Expr, ast::Collect, token_stream::TokenStream};

pub struct ParserUtils;
impl ParserUtils {
    pub fn id(token_stream: &mut TokenStream) -> String {
        match token_stream.skip().value.clone() {
            TokenValue::Id(value) => value,
            _ => panic!("expected id"),
        }
    }

    pub fn op(token_stream: &mut TokenStream) -> Operator {
        match token_stream.skip().value.clone() {
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
