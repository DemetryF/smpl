use derive_more::Constructor;

use crate::{
    error::Error,
    lexer::{Pos, Token, TokenValue},
    TokenStream,
};

use super::Collect;

#[derive(Constructor, Debug, PartialEq, Clone)]
pub struct Id {
    pub id: String,
    pub pos: Pos,
}

impl TryFrom<Token> for Id {
    type Error = Error;

    fn try_from(token: Token) -> Result<Self, Self::Error> {
        match token.value {
            TokenValue::Id(id) => {
                let id = Id::new(id, token.pos);

                Ok(id)
            }

            _ => Err(Error::unexpected_token(token)),
        }
    }
}

impl TryFrom<&Token> for Id {
    type Error = Error;

    fn try_from(token: &Token) -> Result<Self, Self::Error> {
        let token = token.clone();

        Id::try_from(token)
    }
}

impl Collect for Id {
    fn collect(token_stream: &mut TokenStream) -> Result<Self, Error> {
        Id::try_from(token_stream.next())
    }
}

impl std::fmt::Display for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.id)
    }
}
