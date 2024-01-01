mod atom;
mod display;
mod id;
mod label;

use smplc_macros::{display, EnumWrap};

pub use smplc_ast::{BinOp, UnOp};

pub use self::atom::Atom;
pub use self::id::{FunctionId, Id};
pub use self::label::Label;

#[derive(Clone, EnumWrap)]

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
    Halt(Halt),
}

#[derive(Clone)]
#[display("{result} = {lhs} {op} {rhs}")]
pub struct Binary {
    pub result: Id,

    pub lhs: Atom,
    pub op: BinOp,
    pub rhs: Atom,
}

#[derive(Clone)]
#[display("{result} = {op} {rhs}")]
pub struct Unary {
    pub result: Id,

    pub op: UnOp,
    pub rhs: Atom,
}

#[derive(Clone)]
#[display("{result} = {value}")]
pub struct Copy {
    pub result: Id,
    pub value: Atom,
}

#[derive(Clone)]
#[display("if {condition} goto {label}")]
pub struct If {
    pub condition: Atom,
    pub label: Label,
}

#[derive(Clone)]
#[display("unless {condition} goto {label}")]
pub struct Unless {
    pub condition: Atom,
    pub label: Label,
}

#[derive(Clone)]
#[display("goto {label}")]
pub struct Goto {
    pub label: Label,
}

#[derive(Clone)]
pub struct Call {
    pub result: Option<Id>,
    pub id: FunctionId,
}

impl std::fmt::Display for Call {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(result) = &self.result {
            write!(f, "{result} = ")?;
        }

        write!(f, "call {}", self.id)
    }
}

#[derive(Clone)]
#[display("push {value}")]
pub struct Param {
    pub value: Atom,
}

#[derive(Clone)]
#[display("halt")]
pub struct Halt;

#[derive(Clone)]
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
