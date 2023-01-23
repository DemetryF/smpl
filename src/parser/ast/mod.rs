use derive_more::Constructor;

use crate::lexer::pos::Pos;

use super::token_stream::TokenStream;

pub mod block;
pub mod expr;
pub mod statement;

pub trait Collect {
    fn collect(token_stream: &mut TokenStream) -> Self;
}

#[derive(Debug, Constructor)]
pub struct Id {
    pub value: String,
    pub pos: Pos,
}
