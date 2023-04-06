use crate::lexer::token::Literal;

use super::{
    id::Id,
    operators::{BinOp, UnOp},
};

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
    Call {
        id: Id,
        args: Vec<Expr>,
    },
    Atom(Atom),
}

#[derive(PartialEq, Debug)]
pub enum Atom {
    Id(Id),
    Literal(Literal),
}
