mod block;
mod declaration;
mod expr;
mod operators;
mod statement;

use crate::error::ParseResult;
use crate::TokenStream;

pub trait Parse<'source>: Sized {
    fn parse(token_stream: &mut TokenStream<'source>) -> ParseResult<'source, Self>;
}

pub trait TryParse<'source>: Sized {
    fn try_parse(token_stream: &mut TokenStream<'source>) -> Option<Self>;
}
