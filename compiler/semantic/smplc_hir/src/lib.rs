mod expr;
mod statement;

pub use expr::*;
pub use statement::*;

#[derive(Default)]
pub struct HIR<'source> {
    pub constants: Vec<Constant<'source>>,
    pub functions: Vec<Function<'source>>,
}

pub struct Function<'source> {
    pub data: FunRef<'source>,
    pub args: Vec<VarRef<'source>>,
    pub body: Vec<Statement<'source>>,
}

pub struct Constant<'source> {
    pub data: VarRef<'source>,
    pub value: Expr<'source>,
}

pub struct Block<'source> {
    pub statements: Vec<Statement<'source>>,
}
