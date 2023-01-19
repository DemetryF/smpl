use crate::lexer::token::token_value::TokenValue;

use super::{ast::expr::Expr, ast::Collect, token_stream::TokenStream};

pub struct ParserUtils;
impl ParserUtils {
    pub fn id(token_stream: &mut TokenStream) -> String {
        match token_stream.current().value.clone() {
            TokenValue::Id(value) => {
                token_stream.skip();
                value
            }
            _ => panic!("expected id"),
        }
    }

    pub fn parenthesis(token_stream: &mut TokenStream) -> Expr {
        token_stream.accept(&TokenValue::OpeningParen);
        let expr = Expr::collect(token_stream);
        token_stream.accept(&TokenValue::ClosingParen);

        expr
    }
}
