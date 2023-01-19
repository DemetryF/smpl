use super::token_stream::TokenStream;

pub mod block;
pub mod declare_statement;
pub mod expr;
pub mod function_statement;
pub mod if_statement;
pub mod return_statement;
pub mod statement;
pub mod while_statement;

pub trait Collect {
    fn collect(token_stream: &mut TokenStream) -> Self;
}
