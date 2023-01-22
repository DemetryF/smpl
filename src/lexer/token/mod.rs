use derive_more::Constructor;

use self::token_value::TokenValue;
use super::pos::Pos;

pub mod fmt;
pub mod operator;
pub mod token_value;

#[derive(Clone, Debug, Constructor)]
pub struct Token {
    pub value: TokenValue,
    pub pos: Pos,
}
