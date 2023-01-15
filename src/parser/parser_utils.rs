use crate::lexer::token::token_value::TokenValue;

use super::{ast::Expr, collectors::expr_collector::ExprCollector, token_stream::TokenStream};

pub struct ParserUtils;
impl ParserUtils {
    pub fn id<'code>(token_stream: &mut TokenStream<'code>) -> &'code str {
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
        let expr = ExprCollector::collect(token_stream);
        token_stream.accept(&TokenValue::ClosingParen);

        expr
    }
}
