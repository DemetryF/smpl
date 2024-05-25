use std::fmt::{self, Write};

use lir::NumberType;
use smplc_lir::{self as lir, If};

use crate::builder::Builder;
use crate::env::Env;

use crate::compile::Compile;

use super::to_asm;

impl Compile for If {
    fn compile(self, env: &mut Env, builder: &mut Builder) -> fmt::Result {
        let (then_instr, else_instr) = match (self.ty, self.op) {
            (_, lir::RelOp::Eq) => ("je", "jne"),
            (_, lir::RelOp::Ne) => ("jne", "jne"),

            (NumberType::Real, lir::RelOp::Le) => ("jbe", "jae"),
            (NumberType::Real, lir::RelOp::Lt) => ("jb", "ja"),
            (NumberType::Real, lir::RelOp::Gt) => ("ja", "jb"),
            (NumberType::Real, lir::RelOp::Ge) => ("jae", "jbe"),

            (NumberType::Int, lir::RelOp::Le) => ("jle", "jge"),
            (NumberType::Int, lir::RelOp::Lt) => ("jl", "jg"),
            (NumberType::Int, lir::RelOp::Gt) => ("jg", "jl"),
            (NumberType::Int, lir::RelOp::Ge) => ("jge", "jle"),
        };

        let lhs = to_asm(env, builder, self.lhs);
        let rhs = to_asm(env, builder, self.rhs);

        match self.ty {
            NumberType::Real => {
                writeln!(builder, "movss xmm0, {lhs}")?;
                writeln!(builder, "movss xmm1, {rhs}")?;
                writeln!(builder, "ucomiss xmm0, xmm1")?;
            }
            NumberType::Int => {
                writeln!(builder, "mov eax, {lhs}")?;
                writeln!(builder, "mov ebx, {rhs}")?;
                writeln!(builder, "cmp eax, ebx")?;
            }
        }

        if let Some(then_label) = self.then_label {
            writeln!(builder, "{then_instr} {then_label}")?;
        }

        if let Some(else_label) = self.else_label {
            writeln!(builder, "{else_instr} {else_label}")?;
        }

        Ok(())
    }
}
