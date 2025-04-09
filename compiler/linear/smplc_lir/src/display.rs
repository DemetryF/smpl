use std::fmt;

use crate::{
    ArithmOp, Atom, BinOp, Component, ControlFlow, F32sOp, FunId, Id, Phi, Sequental, Type, UnOp,
    Value,
};

impl fmt::Display for Sequental<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Sequental::Assign { dst, value } => {
                writeln!(f, "{dst} = {value}")
            }

            Sequental::Binary { dst, op, lhs, rhs } => {
                writeln!(f, "{dst} = {op} {lhs}, {rhs}")
            }

            Sequental::Unary { dst, op, operand } => {
                writeln!(f, "{dst} = {op} {operand}")
            }

            Sequental::Call { dst, fun, args } => {
                if let Some(dst) = dst {
                    write!(f, "{dst} = ")?;
                }

                write!(f, "call {fun}")?;

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
                rhs,
                label,
            } => {
                writeln!(f, "if {op} {lhs}, {rhs} goto @{}", label.0)
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
            UnOp::Neg(ty) => write!(f, "{ty}.-"),
            UnOp::Swizzle(swizzle) => {
                write!(f, "vec.:")?;

                for &comp in swizzle.as_slice() {
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
            Type::Real => write!(f, "real"),
            Type::Int => write!(f, "int"),
            Type::F32x2 => write!(f, "f32x2"),
            Type::F32x3 => write!(f, "f32x3"),
            Type::F32x4 => write!(f, "f32x4"),
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
            Value::Real(v) => write!(f, "{v}"),
            Value::Int(v) => write!(f, "{v}"),
            Value::F32x2(v) => write!(f, "{v}"),
            Value::F32x3(v) => write!(f, "{v}"),
            Value::F32x4(v) => write!(f, "{v}"),
        }
    }
}

impl fmt::Display for Id {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "${}", self.0)
    }
}

impl fmt::Display for FunId<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Display for BinOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BinOp::Int(op) => write!(f, "int.{op}"),
            BinOp::Real(op) => write!(f, "real.{op}"),
            &BinOp::F32s(dims, op) => write!(f, "f32x{dims}.{op}", dims = dims as usize),
            BinOp::ComplexMul => write!(f, "f32x2.complex_mul"),
            BinOp::ComplexDiv => write!(f, "f32x2.complex_div"),
        }
    }
}

impl fmt::Display for ArithmOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Add => write!(f, "+"),
            Self::Sub => write!(f, "-"),
            Self::Mul => write!(f, "*"),
            Self::Div => write!(f, "/"),
            Self::Eq => write!(f, "=="),
            Self::Ne => write!(f, "!="),
            Self::Lt => write!(f, "<"),
            Self::Le => write!(f, "<="),
            Self::Gt => write!(f, ">"),
            Self::Ge => write!(f, ">="),
        }
    }
}

impl fmt::Display for F32sOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Add => write!(f, "+"),
            Self::Sub => write!(f, "-"),
            Self::ScalarMul => write!(f, "scalar_mul"),
            Self::ScalarDiv => write!(f, "scalar_div"),
            Self::Eq => write!(f, "=="),
            Self::Ne => write!(f, "!="),
        }
    }
}

impl fmt::Display for Component {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Component::X => write!(f, "x"),
            Component::Y => write!(f, "y"),
            Component::Z => write!(f, "z"),
            Component::W => write!(f, "w"),
        }
    }
}
