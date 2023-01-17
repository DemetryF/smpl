use crate::{lexer::token::token_value::TokenValue, parser::token_stream::TokenStream};

use super::{
    declare_statement::DeclareStatement, expr::Expr, function_statement::FunctionStatement,
    if_statement::IfStatement, return_statement::ReturnStatement, while_statement::WhileStatement,
    Collect,
};

#[derive(Debug)]
pub enum Statement<'code> {
    Expr(Expr<'code>),
    Declare(DeclareStatement<'code>),
    Function(FunctionStatement<'code>),
    If(IfStatement<'code>),
    Return(ReturnStatement<'code>),
    While(WhileStatement<'code>),
}

impl<'code> Collect<'code> for Statement<'code> {
    fn collect(token_stream: &mut TokenStream<'code>) -> Self {
        match token_stream.current().value {
            TokenValue::Define => Self::Declare(DeclareStatement::collect(token_stream)),
            TokenValue::If => Self::If(IfStatement::collect(token_stream)),
            TokenValue::Function => Self::Function(FunctionStatement::collect(token_stream)),
            TokenValue::While => Self::While(WhileStatement::collect(token_stream)),
            TokenValue::Return => Self::Return(ReturnStatement::collect(token_stream)),

            _ => {
                let expr = Self::Expr(Expr::collect(token_stream));
                token_stream.accept(&TokenValue::Semicolon);
                expr
            }
        }
    }
}
