mod expr;
mod statement;
mod symbols;
mod ty;

use smplc_ast::Spanned;

pub use expr::*;
pub use statement::*;
pub use symbols::*;
pub use ty::Type;

#[derive(Default)]
pub struct HIR<'source> {
    pub symbols: Symbols<'source>,

    pub constants: Vec<Constant<'source>>,
    pub functions: Vec<Function<'source>>,
}

pub struct Function<'source> {
    pub id: FunId,
    pub args: Vec<VarId>,
    pub body: Block<'source>,
}

pub struct Constant<'source> {
    pub id: VarId,
    pub ty: Type,
    pub value: Spanned<Expr<'source>>,
}

pub struct Block<'source> {
    pub statements: Vec<Statement<'source>>,
}
