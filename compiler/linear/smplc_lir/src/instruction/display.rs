use std::fmt;

use crate::{AssignRhs, Call, FnId, Id, Instruction, Label, Operand, Type};

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Instruction::Assign { res, rhs } => {
                writeln!(f, "{res} = {rhs}")
            }

            Instruction::Goto(label) => {
                writeln!(f, "goto {label}")
            }

            Instruction::IfRel {
                ty,
                op,
                lhs,
                rhs,
                label,
                else_label,
            } => {
                write!(f, "if {ty}.{op} {lhs}, {rhs} goto {label}")?;

                if let Some(label) = else_label {
                    write!(f, ", else {label}")?;
                }

                writeln!(f)
            }

            Instruction::Call(call) => {
                return writeln!(f, "{call}");
            }

            Instruction::Ret(value) => {
                write!(f, "ret")?;

                if let Some(value) = value {
                    write!(f, " {value}")?;
                }

                writeln!(f)
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

            AssignRhs::Phi {
                branches,
                else_value,
            } => {
                write!(f, "phi ")?;

                let mut branches = branches.iter();

                if let Some((label, op)) = branches.next() {
                    write!(f, "{label}: {op}")?;
                }

                branches.try_for_each(|(label, op)| write!(f, ", {label}: {op}"))?;

                if let Some(else_value) = else_value {
                    write!(f, ", {else_value}")?;
                }

                Ok(())
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
