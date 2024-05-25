use std::fmt::{self, Write};

use smplc_lir::{NumberType, Return};

use crate::builder::Builder;
use crate::compile::{to_asm_with_ty, Compile};
use crate::env::Env;

impl Compile for Return {
    fn compile(self, env: &mut Env, builder: &mut Builder) -> fmt::Result {
        if let Some(operand) = self.value {
            let (operand, ty) = to_asm_with_ty(env, builder, operand);

            match ty {
                NumberType::Real => {
                    writeln!(builder, "movss xmm0, {operand}")?;
                }
                NumberType::Int => {
                    writeln!(builder, "mov eax, {operand}")?;
                }
            }
        }

        writeln!(builder, "pop rbp")?;
        writeln!(builder, "ret")
    }
}
