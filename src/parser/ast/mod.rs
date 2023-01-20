use super::token_stream::TokenStream;

pub mod block;
pub mod expr;
pub mod statement;

pub trait Collect {
    fn collect(token_stream: &mut TokenStream) -> Self;
}
