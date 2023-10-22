pub use smplc_lexer::token::{Literal, Pos};

pub mod expr;
pub mod operators;
pub mod statements;

pub use expr::Expr;
pub use statements::Statement;

#[derive(PartialEq, Debug)]
pub struct Block<'source> {
    pub statements: Vec<Statement<'source>>,
}
