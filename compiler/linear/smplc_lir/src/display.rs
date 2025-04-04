use std::fmt;

use crate::{Atom, ControlFlow, Id, Phi, Sequental, Type, UnOp, Value};

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
            write!(f, ", {}", branch)?;
        }

        writeln!(f)
    }
}

impl fmt::Display for UnOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UnOp::Neg => write!(f, "-"),
            UnOp::Swizzle(swizzle) => {
                write!(f, ".")?;

                for &comp in &swizzle.combination[..] {
                    write!(f, "{comp}")?;
                }

                Ok(())
            }
        }
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Type::Complex => write!(f, "complex"),
            Type::Real => write!(f, "real"),
            Type::Int => write!(f, "int"),
            Type::Vec2 => write!(f, "vec2"),
            Type::Vec3 => write!(f, "vec3"),
            Type::Vec4 => write!(f, "vec4"),
        }
    }
}

impl fmt::Display for Atom {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Atom::Value(value) => write!(f, "{value}"),
            Atom::Id(id) => write!(f, "{id}"),
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Complex(v) => write!(f, "{v}"),
            Value::Real(v) => write!(f, "{v}"),
            Value::Int(v) => write!(f, "{v}"),
            Value::Vec2(v) => write!(f, "{v}"),
            Value::Vec3(v) => write!(f, "{v}"),
            Value::Vec4(v) => write!(f, "{v}"),
        }
    }
}

impl fmt::Display for Id {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "${}", self.0)
    }
}
