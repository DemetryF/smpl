use derive_more::Constructor;

use crate::token::Pos;

pub use self::expr::*;
pub use self::statements::*;

pub mod expr;
pub mod statements;

#[derive(Constructor, Clone, Debug)]
pub struct Id {
    pub value: String,
    pub pos: Pos,
}

pub struct Block(pub Vec<Statement>);
