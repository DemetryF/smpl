use super::Id;
use crate::lexer::{Literal, Operator};
use derive_more::Constructor;

pub enum Expr {
    Infix(Infix),
    Prefix(Prefix),
    Call(Call),
    Atom(Atom),
}

#[derive(Constructor)]
pub struct Infix {
    pub lhs: Box<Expr>,
    pub op: Operator,
    pub rhs: Box<Expr>,
}

#[derive(Clone)]
pub enum Atom {
    Literal(Literal),
    Temp(usize),
    Id(Id),
}

#[derive(Constructor)]
pub struct Prefix {
    pub op: Operator,
    pub rhs: Box<Expr>,
}

#[derive(Constructor)]
pub struct Call {
    pub id: Id,
    pub args: Vec<Expr>,
}
