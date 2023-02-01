pub mod expr;
pub mod statements;

use crate::lexer::Pos;
use derive_more::Constructor;

pub use self::expr::*;
pub use self::statements::*;

#[derive(Constructor, Clone)]
pub struct Id {
    pub value: String,
    pub pos: Pos,
}

pub struct Block(pub Vec<Statement>);
