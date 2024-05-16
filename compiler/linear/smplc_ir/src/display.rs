use std::fmt;

use crate::{Assign, AssignRhs, Atom, Call, FunctionId, Instruction, Label, Return};

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Instruction::If(a) => a.fmt(f),
            Instruction::Unless(a) => a.fmt(f),
            Instruction::Goto(a) => a.fmt(f),
            Instruction::Halt(a) => a.fmt(f),

            Instruction::Assign(Assign { lhs, rhs }) => {
                write!(f, "{} = ", lhs)?;

                match rhs {
                    AssignRhs::Binary { lhs, op, rhs } => {
                        write!(f, "{lhs} {op} {rhs}")
                    }

                    AssignRhs::Unary { op, rhs } => {
                        write!(f, "{op} {rhs}")
                    }

                    AssignRhs::Atom(atom) => {
                        write!(f, "{atom}")
                    }

                    AssignRhs::Call(call) => call.fmt(f),
                }
            }

            Instruction::Return(Return { value }) => {
                write!(f, "return")?;

                if let Some(value) = &value {
                    write!(f, " {value}")?;
                }

                Ok(())
            }

            Instruction::Call(call) => call.fmt(f),
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

impl fmt::Display for Call {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "call @{}, ", self.id)?;

        self.args.iter().try_for_each(|arg| write!(f, " {arg}"))
    }
}
