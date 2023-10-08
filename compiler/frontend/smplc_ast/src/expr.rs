use smplc_lexer::token::Pos;

use crate::operators::{BinOp, UnOp};

pub use smplc_lexer::token::Literal;

#[derive(PartialEq, Debug)]
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

#[derive(PartialEq, Debug)]
pub enum Atom<'source> {
    Ident(Ident<'source>),
    Literal(Literal),
}

#[derive(Debug, Clone, Copy)]
pub struct Ident<'source> {
    pub value: &'source str,
    pub pos: Pos,
}

impl PartialEq for Ident<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}
