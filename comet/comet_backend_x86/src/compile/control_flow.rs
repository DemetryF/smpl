use std::fmt::Write;

use comet_ir::{ArithmOp, BinOp, Dims, F32sOp};
use comet_ir::{ControlFlow, Type};

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
                    BinOp::Int(
                        op @ (ArithmOp::Eq
                        | ArithmOp::Ne
                        | ArithmOp::Lt
                        | ArithmOp::Le
                        | ArithmOp::Gt
                        | ArithmOp::Ge),
                    ) => {
                        let cc = match op {
                            ArithmOp::Eq => "e",
                            ArithmOp::Ne => "ne",
                            ArithmOp::Lt => "l",
                            ArithmOp::Le => "le",
                            ArithmOp::Gt => "g",
                            ArithmOp::Ge => "ge",
                            _ => unreachable!(),
                        };

                        writeln!(builder, "mov eax, {lhs}")?;
                        writeln!(builder, "cmp eax, {rhs}")?;
                        writeln!(builder, "j{cc} {}", env.labels[&label])?;
                    }

                    BinOp::Real(op) => {
                        let cc = match op {
                            ArithmOp::Eq => "e",
                            ArithmOp::Ne => "ne",
                            ArithmOp::Lt => "b",
                            ArithmOp::Le => "be",
                            ArithmOp::Gt => "a",
                            ArithmOp::Ge => "ae",
                            _ => unreachable!(),
                        };

                        writeln!(builder, "movss xmm0, {lhs}")?;
                        writeln!(builder, "ucomiss xmm0, {rhs}")?;
                        writeln!(builder, "j{cc} {}", env.labels[&label])?;
                    }

                    BinOp::F32s(dims, op @ (F32sOp::Eq | F32sOp::Ne)) => {
                        let cc = match op {
                            F32sOp::Eq => "e",
                            F32sOp::Ne => "ne",
                            _ => unreachable!(),
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
                        writeln!(builder, "j{cc} {}", env.labels[&label])?;
                    }

                    _ => unreachable!(),
                }
            }

            ControlFlow::Goto { label } => {
                writeln!(builder, "jmp {}", env.labels[&label])?;
            }

            ControlFlow::Return { value } => {
                if let Some(operand) = value {
                    let ty = operand.ty();
                    let operand = atom(env, builder, operand);

                    match ty {
                        Type::Real => {
                            writeln!(builder, "movss xmm0, {operand}")?;
                        }
                        Type::Int => {
                            writeln!(builder, "mov eax, {operand}")?;
                        }
                        _ => {
                            writeln!(builder, "movaps xmm0, {operand}")?;
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
