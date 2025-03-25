mod expr;
mod statement;
mod symbols;

use smplc_ast::Spanned;

pub use expr::*;
pub use statement::*;
pub use symbols::*;

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

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum Type {
    Real,
    Int,
    Bool,
}

impl TryFrom<&str> for Type {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "real" => Ok(Self::Real),
            "int" => Ok(Self::Int),
            "bool" => Ok(Self::Bool),

            _ => Err(()),
        }
    }
}
