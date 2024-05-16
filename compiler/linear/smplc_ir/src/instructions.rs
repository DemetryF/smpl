use smplc_macros::{display, EnumWrap};

use crate::*;

#[derive(EnumWrap)]
pub enum Instruction {
    Assign(Assign),

    If(If),
    Unless(Unless),
    Goto(Goto),
    Call(Call),

    Return(Return),
    Halt(Halt),
}

pub struct Assign {
    pub lhs: Id,
    pub rhs: AssignRhs,
}

pub enum AssignRhs {
    Binary { lhs: Atom, op: BinOp, rhs: Atom },
    Unary { op: UnOp, rhs: Atom },
    Call(Call),
    Atom(Atom),
}

#[display("if {cond} goto {label}")]
pub struct If {
    pub cond: Atom,
    pub label: Label,
}

#[display("unless {cond} goto {label}")]
pub struct Unless {
    pub cond: Atom,
    pub label: Label,
}

#[display("goto {label}")]
pub struct Goto {
    pub label: Label,
}

pub struct Call {
    pub id: FunctionId,
    pub args: Vec<Atom>,
}

#[display("halt")]
pub struct Halt;

pub struct Return {
    pub value: Option<Atom>,
}
