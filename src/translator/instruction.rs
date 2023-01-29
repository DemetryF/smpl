use derive_more::Constructor;
use enum_dispatch::enum_dispatch;

use crate::{lexer::token::operator::Operator, parser::ast::expr::Atom};

#[enum_dispatch(Compile)]
pub enum Instruction {
    Binary,
    Unary,
    Assign,
    Goto,
    Unless,
    Call,
    Label,
    Return,
    Push,
    Pop,
}

#[derive(Clone)]
pub struct Label(pub String);

#[derive(Constructor)]
pub struct Binary {
    pub result: Atom,
    pub left: Atom,
    pub op: Operator,
    pub right: Atom,
}

#[derive(Constructor)]
pub struct Unary {
    pub result: Atom,
    pub op: Operator,
    pub operand: Atom,
}

#[derive(Constructor)]
pub struct Assign {
    pub what: Atom,
    pub op: Operator,
    pub to: Atom,
}

#[derive(Constructor)]
pub struct Goto {
    pub to: Label,
}

#[derive(Constructor)]
pub struct Unless {
    pub cond: Atom,
    pub to: Label,
}

#[derive(Constructor)]
pub struct Call {
    pub result: Atom,
    pub name: String,
    pub args_count: usize,
}

#[derive(Constructor)]
pub struct Return(pub Option<Atom>);

#[derive(Constructor)]
pub struct Push(pub Atom);

#[derive(Constructor)]
pub struct Pop(pub String);
