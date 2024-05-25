use std::fmt::{self, Write};

use smplc_lir::{Call, NumberType};

use crate::builder::Builder;
use crate::env::Env;

use crate::compile::Compile;

use super::to_asm;

impl Compile for Call {
    fn compile(self, env: &mut Env, builder: &mut Builder) -> fmt::Result {
        let shift = env.stack_size() + self.args.len() * 8;

        for (n, (arg, ty)) in self.args.into_iter().rev().enumerate() {
            let value = to_asm(env, builder, arg);
            let address = env.stack_size() + (n + 1) * 8;

            match ty {
                NumberType::Real => {
                    writeln!(builder, "movss xmm0, {value}")?;
                    writeln!(builder, "movss DWORD [rsp - {address}], xmm0")?;
                }
                NumberType::Int => {
                    writeln!(builder, "mov eax, {value}")?;
                    writeln!(builder, "mov DWORD [rsp - {address}], eax")?;
                }
            }
        }

        writeln!(builder, "sub rsp, {shift}")?;
        writeln!(builder, "call {}", self.id)?;
        writeln!(builder, "add rsp, {shift}")
    }
}
