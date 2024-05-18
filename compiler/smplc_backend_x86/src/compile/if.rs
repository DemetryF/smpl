use std::fmt::{self, Write};

use smplc_lir::{self as lir, If};

use crate::builder::Builder;
use crate::env::Env;

use crate::compile::Compile;

use super::to_asm;

impl Compile for If {
    fn compile(self, env: &mut Env, builder: &mut Builder) -> fmt::Result {
        let (then_instr, else_instr) = match self.op {
            lir::RelOp::Eq => ("je", "jne"),
            lir::RelOp::Ne => ("jne", "jne"),
            lir::RelOp::Le => ("jbe", "jae"),
            lir::RelOp::Lt => ("jb", "ja"),
            lir::RelOp::Ge => ("jae", "jbe"),
            lir::RelOp::Gt => ("ja", "jb"),
        };

        let lhs = to_asm(env, builder, self.lhs);
        let rhs = to_asm(env, builder, self.rhs);

        writeln!(builder, "movss xmm0, {lhs}")?;
        writeln!(builder, "movss xmm1, {rhs}")?;
        writeln!(builder, "ucomiss xmm0, xmm1")?;

        if let Some(then_label) = self.then_label {
            writeln!(builder, "{then_instr} {then_label}")?;
        }

        if let Some(else_label) = self.else_label {
            writeln!(builder, "{else_instr} {else_label}")?;
        }

        Ok(())
    }
}
