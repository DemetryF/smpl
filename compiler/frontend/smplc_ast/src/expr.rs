use crate::{BinOp, Spanned, UnOp};

#[derive(Debug, PartialEq)]
pub enum Expr<'source> {
    Prefix {
        op: UnOp,
        rhs: Box<Spanned<Self>>,
    },
    Infix {
        lhs: Box<Spanned<Self>>,
        op: BinOp,
        rhs: Box<Spanned<Self>>,
    },
    Call(Call<'source>),
    Atom(Atom<'source>),
}

#[derive(Debug, PartialEq)]
pub struct Call<'source> {
    pub id: Id<'source>,
    pub args: Vec<Spanned<Expr<'source>>>,
}

#[derive(PartialEq, Debug)]
pub enum Atom<'source> {
    Id(Id<'source>),
    Literal(Literal<'source>),
}

pub type Id<'source> = Spanned<&'source str>;

#[derive(PartialEq, Eq, Debug)]
pub struct Literal<'source> {
    pub value: &'source str,
    pub ty: LiteralType,
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum LiteralType {
    Complex,
    Real,
    Int,
    Bool,
}
