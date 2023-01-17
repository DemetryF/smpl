use derive_more::Constructor;

use crate::{
    lexer::token::token_value::TokenValue,
    parser::{parser_utils::ParserUtils, token_stream::TokenStream},
};

use super::{block::Block, expr::Expr, Collect};

#[derive(Debug, Constructor)]
pub struct IfStatement<'code> {
    pub cond: Expr<'code>,
    pub then_body: Block<'code>,
    pub else_body: Option<Block<'code>>,
}

impl<'code> Collect<'code> for IfStatement<'code> {
    fn collect(token_stream: &mut TokenStream<'code>) -> Self {
        token_stream.accept(&TokenValue::If);

        let cond = ParserUtils::parenthesis(token_stream);
        let then_body = Block::collect(token_stream);
        let else_body = Self::else_body(token_stream);

        IfStatement::new(cond, then_body, else_body)
    }
}

impl<'code> IfStatement<'code> {
    fn else_body(token_stream: &mut TokenStream<'code>) -> Option<Block<'code>> {
        if token_stream.check(&TokenValue::Else) {
            token_stream.skip();
            Some(Block::collect(token_stream))
        } else {
            None
        }
    }
}
