use std::fmt;

use crate::{AssignRhs, Call, FnId, Id, Instruction, Label, Operand, Type};

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Instruction::Assign { res, rhs } => {
                write!(f, "{res} = {rhs}")
            }

            Instruction::Goto(label) => {
                write!(f, "goto {label}")
            }

            Instruction::IfRel {
                ty,
                op,
                lhs,
                rhs,
                label,
            } => {
                write!(f, "if {ty}.{op} {lhs}, {rhs} goto {label}")?;

                Ok(())
            }

            Instruction::Call(call) => {
                return write!(f, "{call}");
            }

            Instruction::Ret(value) => {
                write!(f, "ret")?;

                if let Some(value) = value {
                    write!(f, " {value}")?;
                }

                Ok(())
            }
        }
    }
}

impl fmt::Display for AssignRhs {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AssignRhs::Arithm { ty, op, lhs, rhs } => {
                write!(f, "{ty}.{op} {lhs}, {rhs}")
            }

            AssignRhs::Neg { ty, rhs } => {
                write!(f, "{ty}.neg {rhs}")
            }

            AssignRhs::Phi(branches) => {
                write!(f, "phi ")?;

                let mut branches = branches.iter();

                if let Some((label, op)) = branches.next() {
                    write!(f, "{label}: {op}")?;
                }

                branches.try_for_each(|(label, op)| write!(f, ", {label}: {op}"))
            }

            AssignRhs::Call(call) => write!(f, "{call}"),
            AssignRhs::Operand(operand) => write!(f, "{operand}"),
        }
    }
}

impl fmt::Display for Call {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "call {}(", self.id)?;

        let mut args = self.args.iter();

        if let Some(arg) = args.next() {
            write!(f, "{arg}")?;
        }

        args.try_for_each(|arg| write!(f, ", {arg}"))?;

        write!(f, ")")
    }
}

impl fmt::Display for Id {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Display for FnId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "@{}", self.0)
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Type::Real => write!(f, "real"),
            Type::Int => write!(f, "int"),
        }
    }
}

impl fmt::Display for Label {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "@{}", self.0)
    }
}

impl fmt::Display for Operand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Operand::Real(num) => write!(f, "real({num})"),
            Operand::Int(num) => write!(f, "int({num})"),
            Operand::Id(id) => write!(f, "%{id}"),
        }
    }
}
