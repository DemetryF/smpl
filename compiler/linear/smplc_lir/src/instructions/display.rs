use std::fmt;

use crate::instructions::*;
use crate::Instruction;

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Instruction::Goto(a) => a.fmt(f),
            Instruction::Halt(a) => a.fmt(f),
            Instruction::Call(call) => call.fmt(f),

            Instruction::If(If {
                lhs,
                op,
                rhs,
                then_label,
                else_label,
            }) => {
                write!(f, "if {lhs} {op} {rhs}")?;

                if let Some(label) = then_label {
                    write!(f, " then {label}")?;
                }

                if let Some(label) = else_label {
                    write!(f, " else {label}")?;
                }

                Ok(())
            }

            Instruction::Assign(Assign { lhs, rhs }) => {
                write!(f, "{lhs} = {rhs}")
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

impl fmt::Display for AssignRhs {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AssignRhs::Binary { lhs, op, rhs } => {
                write!(f, "{lhs} {op} {rhs}")
            }

            AssignRhs::Neg { rhs } => {
                write!(f, "-{rhs}")
            }

            AssignRhs::Atom(atom) => {
                write!(f, "{atom}")
            }

            AssignRhs::Call(call) => call.fmt(f),
        }
    }
}
