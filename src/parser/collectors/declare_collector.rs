use crate::{
    lexer::token::{operator::Operator, token_value::TokenValue},
    parser::{ast::Statement, parser_utils::ParserUtils, token_stream::TokenStream},
};

use super::expr_collector::ExprCollector;

pub struct DeclareStatementCollector;
impl DeclareStatementCollector {
    pub fn collect<'code>(token_stream: &mut TokenStream<'code>) -> Statement<'code> {
        token_stream.accept(&TokenValue::Define);

        let id = ParserUtils::id(token_stream);
        let expr = if token_stream.check(&TokenValue::Operator(Operator::Assignment)) {
            token_stream.skip();
            Some(ExprCollector::collect(token_stream))
        } else {
            None
        };

        token_stream.accept(&TokenValue::Semicolon);

        Statement::Declare { id, expr }
    }
}
