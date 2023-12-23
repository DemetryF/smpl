mod expr;
mod statement;

pub use smplc_ast::{BinOp, UnOp};

pub use expr::*;
pub use statement::*;

#[derive(Default)]
pub struct HIR {
    pub functions: Vec<Function>,
}

pub struct Function {
    pub function: FunRef,
    pub statements: Vec<Statement>,
}

pub struct Block {
    pub statements: Vec<Statement>,
}
