use smplc_ast::Type;
use smplc_lexer::TokenValue;

use crate::error::ParseResult;
use crate::{Parse, TokenStream};

impl<'source> Parse<'source> for Type {
    fn parse(token_stream: &mut TokenStream<'source>) -> ParseResult<'source, Self> {
        if let TokenValue::Type(ty) = token_stream.current().value {
            token_stream.next_token();

            Ok(ty)
        } else {
            Err(token_stream.unexpected_token())
        }
    }
}
