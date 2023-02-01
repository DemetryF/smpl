use super::{Collect, Expr, TokenStream};
use crate::{error::*, lexer::TokenValue};

pub use self::{
    declare_statement::DeclareStatement, function_statement::FunctionStatement,
    if_statement::IfStatement, return_statement::ReturnStatement, while_statement::WhileStatement,
};

pub mod declare_statement;
pub mod function_statement;
pub mod if_statement;
pub mod return_statement;
pub mod while_statement;

pub enum Statement {
    Expr(Expr),
    Declare(DeclareStatement),
    Function(FunctionStatement),
    If(IfStatement),
    Return(ReturnStatement),
    While(WhileStatement),
}

impl Collect for Statement {
    fn collect(token_stream: &mut TokenStream) -> Result<Self> {
        Ok(match token_stream.current().value {
            TokenValue::Define => Self::Declare(DeclareStatement::collect(token_stream)?),
            TokenValue::If => Self::If(IfStatement::collect(token_stream)?),
            TokenValue::Function => Self::Function(FunctionStatement::collect(token_stream)?),
            TokenValue::While => Self::While(WhileStatement::collect(token_stream)?),
            TokenValue::Return => Self::Return(ReturnStatement::collect(token_stream)?),

            _ => {
                let expr = Self::Expr(Expr::collect(token_stream)?);
                token_stream.accept(&TokenValue::Semicolon);
                expr
            }
        })
    }
}
