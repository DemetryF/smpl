pub mod declare_statement;
pub mod expr_statement;
pub mod function_statement;
pub mod if_statement;
pub mod return_statement;
pub mod while_statement;

pub use self::{
    declare_statement::DeclareStatement, expr_statement::ExprStatement,
    function_statement::FunctionStatement, if_statement::IfStatement,
    return_statement::ReturnStatement, while_statement::WhileStatement,
};

use crate::{error::Error, lexer::TokenValue, TokenStream};

use super::Collect;

#[derive(Debug, PartialEq)]
pub enum Statement {
    Declare(DeclareStatement),
    Function(FunctionStatement),
    If(IfStatement),
    While(WhileStatement),
    Expr(ExprStatement),
    Return(ReturnStatement),
}

impl Collect for Statement {
    fn collect(token_stream: &mut TokenStream) -> Result<Self, Error> {
        let stmt = match token_stream.current().value {
            TokenValue::Let => Self::Declare(DeclareStatement::collect(token_stream)?),
            TokenValue::Fn => Self::Function(FunctionStatement::collect(token_stream)?),
            TokenValue::If => Self::If(IfStatement::collect(token_stream)?),
            TokenValue::Return => Self::Return(ReturnStatement::collect(token_stream)?),
            TokenValue::While => Self::While(WhileStatement::collect(token_stream)?),

            _ => Self::Expr(ExprStatement::collect(token_stream)?),
        };

        Ok(stmt)
    }
}
