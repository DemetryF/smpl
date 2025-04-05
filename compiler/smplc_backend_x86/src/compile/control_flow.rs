use std::fmt::Write;

use smplc_lir as lir;
use smplc_lir::{ControlFlow, Type};

use crate::{builder::Builder, env::Env};

use super::{atom, Compile};

impl Compile for ControlFlow {
    fn compile(self, env: &mut Env, builder: &mut Builder) -> std::fmt::Result {
        match self {
            ControlFlow::If {
                lhs,
                op: lir::RelOp::Ord(op, ty),
                rhs,
                label,
            } => {
                let jmp_instr = match (ty, op) {
                    (lir::NumberType::Real, lir::OrdOp::Le) => "jbe",
                    (lir::NumberType::Real, lir::OrdOp::Lt) => "jb",
                    (lir::NumberType::Real, lir::OrdOp::Gt) => "ja",
                    (lir::NumberType::Real, lir::OrdOp::Ge) => "jae",

                    (lir::NumberType::Int, lir::OrdOp::Le) => "jle",
                    (lir::NumberType::Int, lir::OrdOp::Lt) => "jl",
                    (lir::NumberType::Int, lir::OrdOp::Gt) => "jg",
                    (lir::NumberType::Int, lir::OrdOp::Ge) => "jge",

                    (lir::NumberType::Complex, _) => unreachable!(),
                };

                let lhs = atom(env, builder, lhs);
                let rhs = atom(env, builder, rhs);

                match ty {
                    lir::NumberType::Real => {
                        writeln!(builder, "movss xmm0, {lhs}")?;
                        writeln!(builder, "movss xmm1, {rhs}")?;
                        writeln!(builder, "ucomiss xmm0, xmm1")?;
                    }
                    lir::NumberType::Int => {
                        writeln!(builder, "mov eax, {lhs}")?;
                        writeln!(builder, "mov ebx, {rhs}")?;
                        writeln!(builder, "cmp eax, ebx")?;
                    }
                    _ => unreachable!(),
                }

                writeln!(builder, "{jmp_instr} {}", env.labels[&label])
            }

            ControlFlow::If {
                lhs,
                op: lir::RelOp::Eq(op, ty),
                rhs,
                label,
            } => {
                let lhs = atom(env, builder, lhs);
                let rhs = atom(env, builder, rhs);

                let op = match op {
                    lir::EqOp::Eq => "je",
                    lir::EqOp::Ne => "jne",
                };

                let label = &env.labels[&label];

                match ty {
                    lir::LinearType::Number(lir::NumberType::Int) => {}

                    lir::LinearType::Number(lir::NumberType::Real) => {
                        writeln!(builder, "movss   xmm0, {lhs}")?;
                        writeln!(builder, "movss   xmm1, {rhs}")?;
                        writeln!(builder, "ucomiss xmm0, xmm1")?;
                        writeln!(builder, "{op} {label}")?;
                    }

                    lir::LinearType::Number(lir::NumberType::Complex)
                    | lir::LinearType::Vec(lir::VecType::Vec2) => {
                        writeln!(builder, "movlps  xmm0, {lhs}")?;
                        writeln!(builder, "movlps  xmm1, {rhs}")?;
                        writeln!(builder, "cmpeqps xmm0, xmm1")?;
                        writeln!(builder, "movmskps eax, xmm0")?;
                        writeln!(builder, "and eax, 0b11")?;
                        writeln!(builder, "cmp eax, 0b11")?;
                        writeln!(builder, "{op} {label}")?;
                    }

                    lir::LinearType::Vec(lir::VecType::Vec3) => {
                        writeln!(builder, "movups  xmm0, {lhs}")?;
                        writeln!(builder, "movups  xmm1, {rhs}")?;
                        writeln!(builder, "cmpeqps xmm0, xmm1")?;
                        writeln!(builder, "movmskps eax, xmm0")?;
                        writeln!(builder, "and eax, 0b111")?;
                        writeln!(builder, "cmp eax, 0b111")?;
                        writeln!(builder, "{op} {label}")?;
                    }

                    lir::LinearType::Vec(lir::VecType::Vec4) => {
                        writeln!(builder, "movups  xmm0, {lhs}")?;
                        writeln!(builder, "movups  xmm1, {rhs}")?;
                        writeln!(builder, "cmpeqps xmm0, xmm1")?;
                        writeln!(builder, "movmskps eax, xmm0")?;
                        writeln!(builder, "and eax, 0b1111")?;
                        writeln!(builder, "cmp eax, 0b1111")?;
                        writeln!(builder, "{op} {label}")?;
                    }
                }

                Ok(())
            }

            ControlFlow::Goto { label } => {
                writeln!(builder, "jmp {}", env.labels[&label])
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
                writeln!(builder, "ret")
            }

            ControlFlow::Halt => {
                writeln!(builder, "mov rbx, 0")?;
                writeln!(builder, "mov rax, 1")?;
                writeln!(builder, "syscall")
            }
        }
    }
}
