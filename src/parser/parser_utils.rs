use crate::lexer::token::token_value::{Id, TokenValue};

use super::{ast::expr::Expr, ast::Collect, token_stream::TokenStream};

pub struct ParserUtils;
impl ParserUtils {
    pub fn id<'code>(token_stream: &mut TokenStream<'code>) -> Id<'code> {
        match token_stream.current().value {
            TokenValue::Id(value) => {
                token_stream.skip();
                value
            }
            _ => panic!("expected id"),
        }
    }

    pub fn parenthesis<'code>(token_stream: &mut TokenStream<'code>) -> Expr<'code> {
        token_stream.accept(&TokenValue::OpeningParen);
        let expr = Expr::collect(token_stream);
        token_stream.accept(&TokenValue::ClosingParen);

        expr
    }
}
