mod declaration;
mod display;
mod expr;
mod operators;
mod pos;
mod span;
mod statement;

pub use declaration::*;
pub use expr::*;
pub use operators::*;
pub use pos::Pos;
pub use span::*;
pub use statement::*;

#[derive(PartialEq, Debug)]
pub struct Block<'source> {
    pub statements: Vec<Statement<'source>>,
}

#[derive(Clone, Copy, PartialEq, Debug, Hash, Eq)]
pub enum Type {
    Real,
    Int,
    Bool,
}
