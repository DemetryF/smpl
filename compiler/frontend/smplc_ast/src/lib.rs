mod declaration;
mod display;
mod expr;
mod operators;
mod pos;
mod statement;

pub use declaration::*;
pub use expr::*;
pub use operators::*;
pub use pos::Pos;
pub use statement::*;

#[derive(PartialEq, Debug)]
pub struct Block<'source> {
    pub statements: Vec<Statement<'source>>,
}
