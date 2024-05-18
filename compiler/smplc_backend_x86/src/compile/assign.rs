use std::fmt::{self, Write};

use smplc_lir::{Assign, AssignRhs, BinOp};

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
                };

                let lhs = to_asm(env, builder, lhs);
                let rhs = to_asm(env, builder, rhs);

                writeln!(builder, "movss xmm0, {lhs}")?;
                writeln!(builder, "{instruction} xmm0, {rhs}")?;
                writeln!(builder, "movss {result_ptr}, xmm0")
            }

            AssignRhs::Neg { rhs } => {
                let operand = to_asm(env, builder, rhs);

                writeln!(builder, "pxor xmm0, xmm0")?;
                writeln!(builder, "subss xmm0, {operand}")?;
                writeln!(builder, "movss {result_ptr}, xmm0")
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
