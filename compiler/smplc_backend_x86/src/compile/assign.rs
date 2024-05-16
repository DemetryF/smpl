use std::fmt::{self, Write};

use smplc_ir::{Assign, AssignRhs, BinOp, UnOp};

use crate::{builder::Builder, env::Env};

use super::{to_asm, Compile};

impl Compile for Assign {
    fn compile(self, env: &mut Env, builder: &mut Builder) -> fmt::Result {
        let result_ptr = env.get_or_add(self.lhs);

        match self.rhs {
            AssignRhs::Binary { lhs, op, rhs } => {
                let instruction = match op {
                    BinOp::Add => "addss",
                    BinOp::Sub => "subss",
                    BinOp::Mul => "mulss",
                    BinOp::Div => "divss",

                    BinOp::And => "and",
                    BinOp::Or => "or",

                    BinOp::Eq => "sete",
                    BinOp::Ne => "setne",
                    BinOp::Ge => "setae",
                    BinOp::Gt => "seta",
                    BinOp::Le => "setbe",
                    BinOp::Lt => "setb",
                };

                let lhs = to_asm(env, builder, lhs);
                let rhs = to_asm(env, builder, rhs);

                if op.is_arithm() {
                    writeln!(builder, "movss xmm0, {lhs}")?;
                    writeln!(builder, "{instruction} xmm0, {rhs}")?;
                    writeln!(builder, "movss {result_ptr}, xmm0")?;
                } else if op.is_rel() {
                    writeln!(builder, "movss xmm0, {lhs}")?;
                    writeln!(builder, "movss xmm1, {rhs}")?;

                    writeln!(builder, "ucomiss xmm0, xmm1")?;
                    writeln!(builder, "{instruction} al")?;
                    writeln!(builder, "movzx eax, al")?;
                    writeln!(builder, "cvtsi2ss xmm0, eax")?;
                    writeln!(builder, "movss {result_ptr}, xmm0")?;
                } else if op.is_logic() {
                    let one = builder.float(1.0);

                    // xmm0 = 1
                    writeln!(builder, "movss xmm0, {one}")?;
                    // xmm1 = lhs
                    writeln!(builder, "movss xmm1, {lhs}")?;

                    // eax = 1 == lhs
                    writeln!(builder, "ucomiss xmm0, xmm1")?;
                    writeln!(builder, "sete al")?;
                    writeln!(builder, "movzx eax, al")?;

                    // xmm1 = rhs
                    writeln!(builder, "movss xmm1, {rhs}")?;
                    // ebx = 1 == rhs
                    writeln!(builder, "ucomiss xmm0, xmm1")?;
                    writeln!(builder, "sete bl")?;
                    writeln!(builder, "movzx ebx, bl")?;

                    writeln!(builder, "{instruction} ebx, eax")?;
                    writeln!(builder, "cvtsi2ss xmm0, ebx")?;
                    writeln!(builder, "movss {result_ptr}, xmm0")?;
                }

                Ok(())
            }

            AssignRhs::Unary { op, rhs } => {
                let operand = to_asm(env, builder, rhs);

                match op {
                    UnOp::Not => {
                        writeln!(builder, "xorps xmm0, xmm0")?;
                        writeln!(builder, "ucomiss {operand}, xmm0")?;
                        writeln!(builder, "sete ah")?;
                        writeln!(builder, "cvtsi2ss xmm0, ah")?;
                        writeln!(builder, "movss {result_ptr}, xmm0")
                    }
                    UnOp::Neg => {
                        writeln!(builder, "pxor xmm0, xmm0")?;
                        writeln!(builder, "subss xmm0, {operand}")?;
                        writeln!(builder, "movss {result_ptr}, xmm0")
                    }
                }
            }

            AssignRhs::Call(call) => {
                call.compile(env, builder)?;

                writeln!(builder, "movss {result_ptr}, xmm0")
            }

            AssignRhs::Atom(atom) => {
                let value = to_asm(env, builder, atom);

                writeln!(builder, "movss xmm0, {value}")?;
                writeln!(builder, "movss {result_ptr}, xmm0")
            }
        }
    }
}
