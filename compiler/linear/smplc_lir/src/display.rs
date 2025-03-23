use std::fmt;

use crate::{Atom, ControlFlow, Id, Number, Phi, Sequental, Type, UnOp};

impl fmt::Display for Sequental {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Sequental::Assign { dst, value } => {
                writeln!(f, "{dst} = {value}")
            }
            Sequental::Binary {
                dst,
                op,
                ty,
                lhs,
                rhs,
            } => {
                writeln!(f, "{dst} = {ty}.{op} {lhs}, {rhs}")
            }
            Sequental::Unary {
                dst,
                op,
                ty,
                operand,
            } => {
                writeln!(f, "{dst} = {ty}.{op} {operand}")
            }
            Sequental::Call { dst, args, .. } => {
                if let Some(dst) = dst {
                    write!(f, "{dst} = ")?;
                }

                write!(f, "call @... ")?;

                let mut args = args.iter();

                if let Some((arg, ty)) = args.next() {
                    write!(f, "{ty} {arg}")?;
                }

                for (arg, ty) in args {
                    write!(f, ", {ty} {arg}")?;
                }

                writeln!(f)
            }
        }
    }
}

impl fmt::Display for ControlFlow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ControlFlow::If {
                lhs,
                op,
                ty,
                rhs,
                label,
            } => {
                writeln!(f, "if {ty}.{op} {lhs}, {rhs} goto @{}", label.0)
            }
            ControlFlow::Goto { label } => {
                writeln!(f, "goto {}", label.0)
            }
            ControlFlow::Return { value } => {
                write!(f, "return")?;

                if let Some(value) = value {
                    write!(f, " {value}")?;
                }

                writeln!(f)
            }
            ControlFlow::Halt => {
                writeln!(f, "halt")
            }
        }
    }
}

impl fmt::Display for Phi {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} = ", self.dst)?;

        let mut branches = self.branches.iter();

        if let Some(branch) = branches.next() {
            write!(f, "{}", branch)?;
        }

        for branch in branches {
            write!(f, "{}", branch)?;
        }

        writeln!(f)
    }
}

impl fmt::Display for UnOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UnOp::Neg => write!(f, "-"),
        }
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

impl fmt::Display for Atom {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Atom::Number(Number::Real(value)) => write!(f, "{value}"),
            Atom::Number(Number::Int(value)) => write!(f, "{value}"),
            Atom::Id(id) => write!(f, "{id}"),
        }
    }
}

impl fmt::Display for Id {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "${}", self.0)
    }
}
