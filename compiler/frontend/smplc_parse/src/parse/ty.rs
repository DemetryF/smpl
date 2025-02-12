use smplc_ast::Type;
use smplc_lexer::TokenTag;

use crate::error::ParseResult;
use crate::token_stream::Tokens;
use crate::{Parse, TokenStream};

impl<'source> Parse<'source> for Type {
    fn parse<TS: Tokens<'source>>(
        token_stream: &mut TokenStream<'source, TS>,
    ) -> ParseResult<'source, Self> {
        if let TokenTag::Type(ty) = token_stream.current().tag {
            token_stream.next_token()?;

            Ok(ty)
        } else {
            Err(token_stream.unexpected_token())
        }
    }
}
