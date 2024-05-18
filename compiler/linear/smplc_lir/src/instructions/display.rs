use std::fmt;

use crate::instructions::*;
use crate::Instruction;

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Instruction::If(a) => a.fmt(f),
            Instruction::Unless(a) => a.fmt(f),
            Instruction::Goto(a) => a.fmt(f),
            Instruction::Halt(a) => a.fmt(f),
            Instruction::Call(call) => call.fmt(f),

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
        }
    }
}

impl fmt::Display for Call {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "call @{}, ", self.id)?;

        self.args.iter().try_for_each(|arg| write!(f, " {arg}"))
    }
}
