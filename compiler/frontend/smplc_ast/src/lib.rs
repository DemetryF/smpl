pub use smplc_lexer::token::{Literal, Pos};

pub mod expr;
pub mod operators;
pub mod statement;

pub use expr::Expr;
pub use statement::Statement;

pub struct Block<'source> {
    pub statements: Vec<Statement<'source>>,
}
