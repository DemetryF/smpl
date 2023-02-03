use super::{Collect, TokenStream};
use crate::{ast::*, error::*, lexer::TokenValue};

pub mod declare_statement;
pub mod function_statement;
pub mod if_statement;
pub mod return_statement;
pub mod while_statement;

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
                token_stream.accept(&TokenValue::Semicolon)?;
                expr
            }
        })
    }
}
