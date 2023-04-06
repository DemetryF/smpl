use super::Collect;
use crate::{ast::id::Id, error::Error, parser::token_stream::TokenStream};

impl Collect for Id {
    fn collect(token_stream: &mut TokenStream) -> Result<Self, Error> {
        Id::try_from(token_stream.next())
    }
}
