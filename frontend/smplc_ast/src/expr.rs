use crate::{BinOp, Pos, UnOp};

#[derive(Debug, PartialEq)]
pub enum Expr {
    Prefix {
        op: UnOp,
        rhs: Box<Expr>,
    },
    Infix {
        lhs: Box<Expr>,
        op: BinOp,
        rhs: Box<Expr>,
    },
    Call(Call),
    Atom(Atom),
}

#[derive(Debug, PartialEq)]
pub struct Call {
    pub id: Id,
    pub args: Vec<Expr>,
}

#[derive(PartialEq, Debug)]
pub enum Atom {
    Id(Id),
    Literal(Literal),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Id {
    pub id: String,
    pub pos: Pos,
}

impl Id {
    pub fn new(id: String, pos: Pos) -> Self {
        Self { id, pos }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Literal {
    Number(f32),
    Bool(bool),
}
