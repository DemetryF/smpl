mod expr;
mod statement;

pub use smplc_ast::{BinOp, UnOp};

pub use expr::*;
pub use statement::*;

#[derive(Default)]
pub struct HIR {
    pub constants: Vec<Constant>,
    pub functions: Vec<Function>,
    pub variables_count: usize,
}

pub struct Function {
    pub function: FunRef,
    pub args: Vec<VarRef>,
    pub statements: Vec<Statement>,
}

pub struct Constant {
    pub variable: VarRef,
    pub value: Expr,
}

pub struct Block {
    pub statements: Vec<Statement>,
}
