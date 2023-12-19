mod block;
mod expr;
mod statement;

use crate::{ParseError, TokenStream};

pub trait Collect: Sized {
    fn collect(token_stream: &mut TokenStream) -> Result<Self, ParseError>;
}
