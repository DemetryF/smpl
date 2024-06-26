mod expr;
mod operators;
mod statement;

pub use expr::*;
pub use operators::*;
pub use statement::*;

#[derive(Default)]
pub struct HIR {
    pub constants: Vec<Constant>,
    pub functions: Vec<Function>,
}

pub struct Function {
    pub data: FunRef,
    pub args: Vec<VarRef>,
    pub body: Vec<Statement>,
}

pub struct Constant {
    pub data: VarRef,
    pub value: Expr,
}

pub struct Block {
    pub statements: Vec<Statement>,
}
