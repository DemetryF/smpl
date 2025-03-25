use std::fmt::Write;

use smplc_lir as lir;
use smplc_lir::{ControlFlow, Type};

use crate::{builder::Builder, env::Env};

use super::{to_asm, Compile};

impl Compile for ControlFlow {
    fn compile(self, env: &mut Env, builder: &mut Builder) -> std::fmt::Result {
        match self {
            ControlFlow::If {
                lhs,
                op,
                ty,
                rhs,
                label,
            } => {
                let jmp_instr = match (ty, op) {
                    (_, lir::RelOp::Eq) => "je",
                    (_, lir::RelOp::Ne) => "jne",

                    (Type::Real, lir::RelOp::Le) => "jbe",
                    (Type::Real, lir::RelOp::Lt) => "jb",
                    (Type::Real, lir::RelOp::Gt) => "ja",
                    (Type::Real, lir::RelOp::Ge) => "jae",

                    (Type::Int, lir::RelOp::Le) => "jle",
                    (Type::Int, lir::RelOp::Lt) => "jl",
                    (Type::Int, lir::RelOp::Gt) => "jg",
                    (Type::Int, lir::RelOp::Ge) => "jge",
                };

                let lhs = to_asm(env, builder, lhs);
                let rhs = to_asm(env, builder, rhs);

                match ty {
                    Type::Real => {
                        writeln!(builder, "movss xmm0, {lhs}")?;
                        writeln!(builder, "movss xmm1, {rhs}")?;
                        writeln!(builder, "ucomiss xmm0, xmm1")?;
                    }
                    Type::Int => {
                        writeln!(builder, "mov eax, {lhs}")?;
                        writeln!(builder, "mov ebx, {rhs}")?;
                        writeln!(builder, "cmp eax, ebx")?;
                    }
                }

                writeln!(builder, "{jmp_instr} {}", env.labels[&label])?;

                Ok(())
            }

            ControlFlow::Goto { label } => {
                writeln!(builder, "jmp {}", env.labels[&label])
            }

            ControlFlow::Return { value } => {
                if let Some(operand) = value {
                    let ty = operand.ty();
                    let operand = to_asm(env, builder, operand);

                    match ty {
                        Type::Real => {
                            writeln!(builder, "movss xmm0, {operand}")?;
                        }
                        Type::Int => {
                            writeln!(builder, "mov eax, {operand}")?;
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
