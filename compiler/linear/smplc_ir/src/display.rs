use std::fmt;

use crate::{Atom, FunctionId, Instruction, Label};

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Instruction::Binary(a) => a.fmt(f),
            Instruction::Unary(a) => a.fmt(f),
            Instruction::Copy(a) => a.fmt(f),
            Instruction::If(a) => a.fmt(f),
            Instruction::Unless(a) => a.fmt(f),
            Instruction::Goto(a) => a.fmt(f),
            Instruction::Call(a) => a.fmt(f),
            Instruction::Return(a) => a.fmt(f),
            Instruction::Halt(a) => a.fmt(f),
        }
    }
}

impl fmt::Display for Atom {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Atom::Id(id) => write!(f, "{id}"),
            Atom::Number(num) => write!(f, "{num}"),
        }
    }
}

impl fmt::Display for FunctionId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::fmt::Display for Label {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
