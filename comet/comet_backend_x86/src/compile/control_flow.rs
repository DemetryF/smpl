use std::fmt::Write;

use comet_ir::{BinOp, ControlFlow, Dims, EqOp, RelOp, Type};

use crate::{builder::Builder, env::Env};

use super::{atom, Compile};

impl Compile for ControlFlow {
    fn compile(self, env: &mut Env, builder: &mut Builder) -> std::fmt::Result {
        match self {
            ControlFlow::If {
                lhs,
                op,
                rhs,
                label,
            } => {
                let lhs = atom(env, builder, lhs);
                let rhs = atom(env, builder, rhs);

                match op {
                    BinOp::IntRel(op) => {
                        let cond = match op {
                            RelOp::Eq => "e",
                            RelOp::Ne => "ne",
                            RelOp::Lt => "l",
                            RelOp::Le => "le",
                            RelOp::Gt => "g",
                            RelOp::Ge => "ge",
                        };

                        writeln!(builder, "mov eax, {lhs}")?;
                        writeln!(builder, "cmp eax, {rhs}")?;
                        writeln!(builder, "j{cond} {}", env.labels[&label])?;
                    }

                    BinOp::RealRel(op) => {
                        let cond = match op {
                            RelOp::Eq => "e",
                            RelOp::Ne => "ne",
                            RelOp::Lt => "b",
                            RelOp::Le => "be",
                            RelOp::Gt => "a",
                            RelOp::Ge => "ae",
                        };

                        writeln!(builder, "movss xmm0, {lhs}")?;
                        writeln!(builder, "ucomiss xmm0, {rhs}")?;
                        writeln!(builder, "j{cond} {}", env.labels[&label])?;
                    }

                    BinOp::F32sRel(dims, op) => {
                        let cond = match op {
                            EqOp::Eq => "e",
                            EqOp::Ne => "ne",
                        };

                        let mask = match dims {
                            Dims::X2 => "0b11",
                            Dims::X3 => "0b111",
                            Dims::X4 => "0b1111",
                        };

                        writeln!(builder, "movaps  xmm0, {lhs}")?;
                        writeln!(builder, "movaps  xmm1, {rhs}")?;
                        writeln!(builder, "cmpeqps xmm0, xmm1")?;
                        writeln!(builder, "movmskps eax, xmm0")?;
                        writeln!(builder, "and eax, {mask}")?;
                        writeln!(builder, "cmp eax, {mask}")?;
                        writeln!(builder, "j{cond} {}", env.labels[&label])?;
                    }

                    _ => unreachable!(),
                }
            }

            ControlFlow::Goto { label } => {
                writeln!(builder, "jmp {}", env.labels[&label])?;
            }

            ControlFlow::Return { value } => {
                if let Some((ty, operand)) = value {
                    let operand = atom(env, builder, operand);

                    match ty {
                        Type::Real | Type::F32x2 | Type::F32x3 | Type::F32x4 => {
                            writeln!(builder, "movups xmm0, {operand}")?;
                        }
                        Type::Int => {
                            writeln!(builder, "mov eax, {operand}")?;
                        }
                    }
                }

                writeln!(builder, "pop rbp")?;
                writeln!(builder, "ret")?;
            }

            ControlFlow::Halt => {
                writeln!(builder, "mov rbx, 0")?;
                writeln!(builder, "mov rax, 1")?;
                writeln!(builder, "syscall")?;
            }
        }

        Ok(())
    }
}
