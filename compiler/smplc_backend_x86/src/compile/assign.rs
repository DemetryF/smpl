use std::fmt::{self, Write};

use smplc_lir::{ArithmOp, Assign, AssignRhs, NumberType};

use crate::{builder::Builder, env::Env};

use super::{to_asm, Compile};

impl Compile for Assign {
    fn compile(self, env: &mut Env, builder: &mut Builder) -> fmt::Result {
        let result_ptr = env.get_or_add(self.lhs);

        match self.rhs {
            AssignRhs::Binary { lhs, op, rhs, ty } => {
                let instruction = match op {
                    ArithmOp::Add => "add",
                    ArithmOp::Sub => "sub",
                    ArithmOp::Mul if ty == NumberType::Int => "imul",
                    ArithmOp::Mul => "mul",
                    ArithmOp::Div => "div",
                };

                let lhs = to_asm(env, builder, lhs);
                let rhs = to_asm(env, builder, rhs);

                match ty {
                    NumberType::Real => {
                        writeln!(builder, "movss xmm0, {lhs}")?;
                        writeln!(builder, "{instruction}ss xmm0, {rhs}")?;
                        writeln!(builder, "movss {result_ptr}, xmm0")
                    }
                    NumberType::Int if op == ArithmOp::Div => {
                        writeln!(builder, "mov eax, {lhs}")?;
                        writeln!(builder, "mov ebx, {rhs}")?;
                        writeln!(builder, "idiv ebx")?;
                        writeln!(builder, "mov {result_ptr}, eax")
                    }
                    NumberType::Int => {
                        writeln!(builder, "mov eax, {lhs}")?;
                        writeln!(builder, "mov ebx, {rhs}")?;
                        writeln!(builder, "{instruction} eax, ebx")?;
                        writeln!(builder, "mov {result_ptr}, eax")
                    }
                }
            }

            AssignRhs::Neg { rhs, ty } => {
                let operand = to_asm(env, builder, rhs);

                match ty {
                    NumberType::Real => {
                        writeln!(builder, "pxor xmm0, xmm0")?;
                        writeln!(builder, "subss xmm0, {operand}")?;
                        writeln!(builder, "movss {result_ptr}, xmm0")
                    }
                    NumberType::Int => {
                        writeln!(builder, "xor eax, eax")?;
                        writeln!(builder, "sub eax, {operand}")?;
                        writeln!(builder, "mov {result_ptr}, eax")
                    }
                }
            }

            AssignRhs::Call(call, ty) => {
                call.compile(env, builder)?;

                match ty {
                    NumberType::Real => {
                        writeln!(builder, "movss {result_ptr}, xmm0")
                    }

                    NumberType::Int => {
                        writeln!(builder, "mov {result_ptr}, eax")
                    }
                }
            }

            AssignRhs::Atom(atom) => {
                let value = to_asm(env, builder, atom);

                match env.ty(self.lhs) {
                    NumberType::Real => {
                        writeln!(builder, "movss xmm0, {value}")?;
                        writeln!(builder, "movss {result_ptr}, xmm0")
                    }
                    NumberType::Int => {
                        writeln!(builder, "mov eax, {value}")?;
                        writeln!(builder, "mov {result_ptr}, eax")
                    }
                }
            }
        }
    }
}
