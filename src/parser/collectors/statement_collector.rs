use crate::{
    lexer::token::token_value::TokenValue,
    parser::{ast::Statement, token_stream::TokenStream},
};

use super::{
    declare_collector::DeclareStatementCollector, expr_collector::ExprCollector,
    function_collector::FunctionStatementCollector, if_collector::IfStatementCollector,
    return_collector::ReturnStatementCollector, while_collector::WhileStatementCollector,
};

pub struct StatementCollector;
impl StatementCollector {
    pub fn collect<'code>(token_stream: &mut TokenStream<'code>) -> Statement<'code> {
        match token_stream.current().value {
            TokenValue::Define => DeclareStatementCollector::collect(token_stream),
            TokenValue::If => IfStatementCollector::collect(token_stream),
            TokenValue::Function => FunctionStatementCollector::collect(token_stream),
            TokenValue::While => WhileStatementCollector::collect(token_stream),
            TokenValue::Return => ReturnStatementCollector::collect(token_stream),

            _ => {
                let expr = Statement::Expr(ExprCollector::collect(token_stream));
                token_stream.accept(&TokenValue::Semicolon);
                expr
            }
        }
    }
}
