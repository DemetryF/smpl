mod display;
mod expr;
mod operators;
mod statement;

pub use expr::*;
pub use operators::*;
pub use statement::*;

pub use smplc_lexer::{Literal, Pos};

#[derive(PartialEq, Debug)]
pub struct Block {
    pub stmts: Vec<Statement>,
}
