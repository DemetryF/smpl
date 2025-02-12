mod expr;
mod operators;
mod statement;

pub use expr::*;
pub use operators::*;
pub use statement::*;

#[derive(Default)]
pub struct HIR<'source> {
    pub constants: Vec<Constant<'source>>,
    pub functions: Vec<Function<'source>>,
}

pub struct Function<'source> {
    pub data: FunRef,
    pub args: Vec<VarRef>,
    pub body: Vec<Statement<'source>>,
}

pub struct Constant<'source> {
    pub data: VarRef,
    pub value: Expr<'source>,
}

pub struct Block<'source> {
    pub statements: Vec<Statement<'source>>,
}
