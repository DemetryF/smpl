mod block;
mod declaration;
mod expr;
mod operators;
mod statement;

use crate::{error::ParseResult, token_stream::Tokens, TokenStream};

pub trait Parse<'source>: Sized {
    fn parse<TS: Tokens<'source>>(
        token_stream: &mut TokenStream<'source, TS>,
    ) -> ParseResult<'source, Self>;
}

pub trait TryParse<'source>: Sized {
    fn try_parse<TS: Tokens<'source>>(token_stream: &mut TokenStream<'source, TS>) -> Option<Self>;
}
