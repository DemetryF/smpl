use smplc_lexer::token::{Literal, Pos};

use crate::operators::{BinOp, UnOp};

pub enum Expr<'source> {
    Binary {
        lhs: Box<Self>,
        op: BinOp,
        rhs: Box<Self>,
    },
    Unary {
        op: UnOp,
        rhs: Box<Self>,
    },
    Call {
        id: Ident<'source>,
        args: Vec<Self>,
    },
    Atom(Atom<'source>),
}

pub enum Atom<'source> {
    Ident(Ident<'source>),
    Literal(Literal),
}

#[derive(Clone, Copy)]
pub struct Ident<'source> {
    pub value: &'source str,
    pub pos: Pos,
}
