use derive_more::Constructor;

use crate::{
    error::Error,
    lexer::{
        pos::Pos,
        token::{Token, TokenValue},
    },
};

#[derive(Constructor, Debug, PartialEq)]
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
