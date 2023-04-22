mod atom;
mod display;
mod id;
mod label;

use derive_more::Constructor;
use macros::{display, EnumWrap};

pub use frontend::ast::{BinOp, UnOp};

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

    Param(Param),
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

pub struct Call {
    pub result: Option<Id>,
    pub id: Id,
}

impl std::fmt::Display for Call {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(result) = &self.result {
            write!(f, "{result} = ")?;
        }

        write!(f, "call {}", self.id)
    }
}

#[display("push {value}")]
#[derive(Constructor)]
pub struct Param {
    pub value: Atom,
}

#[display("pop {value}")]
pub struct Pop {
    pub value: Atom,
}

pub struct Return {
    pub value: Option<Atom>,
}

impl std::fmt::Display for Return {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "return")?;

        if let Some(value) = &self.value {
            write!(f, " {value}")?;
        }

        Ok(())
    }
}
