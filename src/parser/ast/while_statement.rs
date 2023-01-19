use derive_more::Constructor;

use crate::{
    lexer::token::token_value::TokenValue,
    parser::{parser_utils::ParserUtils, token_stream::TokenStream},
};

use super::{block::Block, expr::Expr, Collect};

#[derive(Debug, Constructor)]
pub struct WhileStatement {
    pub cond: Expr,
    pub body: Block,
}

impl Collect for WhileStatement {
    fn collect(token_stream: &mut TokenStream) -> Self {
        token_stream.accept(&TokenValue::While);

        let cond = ParserUtils::parenthesis(token_stream);
        let body = Block::collect(token_stream);

        WhileStatement::new(cond, body)
    }
}
