use derive_more::Constructor;

use crate::{
    lexer::token::token_value::TokenValue,
    parser::{parser_utils::ParserUtils, token_stream::TokenStream},
};

use super::{block::Block, expr::Expr, Collect};

#[derive(Debug, Constructor)]
pub struct IfStatement {
    pub cond: Expr,
    pub then_body: Block,
    pub else_body: Option<Block>,
}

impl Collect for IfStatement {
    fn collect(token_stream: &mut TokenStream) -> Self {
        token_stream.accept(&TokenValue::If);

        let cond = ParserUtils::parenthesis(token_stream);
        let then_body = Block::collect(token_stream);
        let else_body = Self::else_body(token_stream);

        IfStatement::new(cond, then_body, else_body)
    }
}

impl IfStatement {
    fn else_body(token_stream: &mut TokenStream) -> Option<Block> {
        if token_stream.check(&TokenValue::Else) {
            token_stream.skip();
            Some(Block::collect(token_stream))
        } else {
            None
        }
    }
}
