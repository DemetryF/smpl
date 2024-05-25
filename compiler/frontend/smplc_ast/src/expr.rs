use crate::{BinOp, Pos, UnOp};

#[derive(Debug, PartialEq)]
pub enum Expr<'source> {
    Prefix {
        op: UnOp,
        rhs: Box<Self>,
    },
    Infix {
        lhs: Box<Self>,
        op: BinOp,
        rhs: Box<Self>,
    },
    Call(Call<'source>),
    Atom(Atom<'source>),
}

#[derive(Debug, PartialEq)]
pub struct Call<'source> {
    pub id: Id<'source>,
    pub args: Vec<Expr<'source>>,
}

#[derive(PartialEq, Debug)]
pub enum Atom<'source> {
    Id(Id<'source>),
    Literal(Literal),
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Id<'source> {
    pub name: &'source str,
    pub pos: Pos,
}

impl<'source> Id<'source> {
    pub fn new(name: &'source str, pos: Pos) -> Self {
        Self { name, pos }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Literal {
    Real(f32),
    Int(i32),
    Bool(bool),
}
