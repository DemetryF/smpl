use crate::{
    lexer::token::token_value::TokenValue,
    parser::{
        ast::{Expr, Statement},
        token_stream::TokenStream,
    },
};

use super::expr_collector::ExprCollector;

pub struct ReturnStatementCollector;
impl ReturnStatementCollector {
    pub fn collect<'code>(token_stream: &mut TokenStream<'code>) -> Statement<'code> {
        token_stream.accept(&TokenValue::Return);

        let expr = Self::return_expr(token_stream);

        token_stream.accept(&TokenValue::Semicolon);

        Statement::Return(expr)
    }

    pub fn return_expr<'code>(token_stream: &mut TokenStream<'code>) -> Option<Expr<'code>> {
        if token_stream.check(&TokenValue::Semicolon) {
            None
        } else {
            Some(ExprCollector::collect(token_stream))
        }
    }
}
