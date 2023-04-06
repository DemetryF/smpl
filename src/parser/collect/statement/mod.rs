use crate::{
    ast::{
        DeclareStatement, ExprStatement, FunctionStatement, IfStatement, ReturnStatement,
        Statement, WhileStatement,
    },
    error::Error,
    lexer::token::TokenValue,
    parser::token_stream::TokenStream,
};

use super::Collect;

mod declare_statement;
mod expr_statement;
mod function_statement;
mod if_statement;
mod return_statement;
mod while_statement;

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
