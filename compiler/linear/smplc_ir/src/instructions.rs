use std::fmt;

use smplc_macros::{display, EnumWrap};

use crate::*;

#[derive(Clone, EnumWrap)]
pub enum Instruction {
    Binary(Binary),
    Unary(Unary),
    Copy(Copy),

    If(If),
    Unless(Unless),
    Goto(Goto),
    Call(Call),

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
#[display("if {cond} goto {label}")]
pub struct If {
    pub cond: Atom,
    pub label: Label,
}

#[derive(Clone)]
#[display("unless {cond} goto {label}")]
pub struct Unless {
    pub cond: Atom,
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
    pub args: Vec<Atom>,
}

impl fmt::Display for Call {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(result) = &self.result {
            write!(f, "{result} = ")?;
        }

        write!(f, "call {}(", self.id)?;

        self.args.iter().try_for_each(|a| write!(f, "{a}"))?;

        write!(f, ")")
    }
}

#[derive(Clone)]
#[display("halt")]
pub struct Halt;

#[derive(Clone)]
pub struct Return {
    pub value: Option<Atom>,
}

impl fmt::Display for Return {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "return")?;

        if let Some(value) = &self.value {
            write!(f, " {value}")?;
        }

        Ok(())
    }
}
