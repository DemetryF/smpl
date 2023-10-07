use smplc_lexer::token::{Literal, Posed};

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

pub type Ident<'source> = Posed<&'source str>;
