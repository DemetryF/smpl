mod atom;
mod display;
mod id;
mod label;

use frontend::ast::{BinOp, UnOp};
use macros::{display, EnumWrap};

pub use self::{atom::Atom, id::Id, label::Label};

#[derive(EnumWrap)]

pub enum Instruction {
    Binary(Binary),
    Unary(Unary),
    Copy(Copy),

    If(If),
    Unless(Unless),
    Goto(Goto),
    Call(Call),

    Push(Push),
    Pop(Pop),
    Return(Return),
}

#[display("{result} = {lhs} {op} {rhs}")]
pub struct Binary {
    pub result: Id,

    pub lhs: Atom,
    pub op: BinOp,
    pub rhs: Atom,
}

#[display("{result} = {op} {rhs}")]
pub struct Unary {
    pub result: Id,

    pub op: UnOp,
    pub rhs: Atom,
}

#[display("{result} = {value}")]
pub struct Copy {
    pub result: Id,
    pub value: Atom,
}

#[display("if {condition} goto {label}")]
pub struct If {
    pub condition: Atom,
    pub label: Label,
}

#[display("unless {condition} goto {label}")]
pub struct Unless {
    pub condition: Atom,
    pub label: Label,
}

#[display("goto {label}")]
pub struct Goto {
    pub label: Label,
}

#[display("{result} = call {id}")]
pub struct Call {
    pub result: Id,
    pub id: Id,
}

#[display("push {value}")]
pub struct Push {
    pub value: Atom,
}

#[display("pop {value}")]
pub struct Pop {
    pub value: Atom,
}

#[display("return {value}")]
pub struct Return {
    pub value: Atom,
}
