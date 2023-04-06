mod block;
mod expr;
mod id;
mod statement;

use crate::error::Error;

use super::token_stream::TokenStream;

pub trait Collect: Sized {
    fn collect(token_stream: &mut TokenStream) -> Result<Self, Error>;
}
