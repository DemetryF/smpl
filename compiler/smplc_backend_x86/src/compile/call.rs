use std::fmt::{self, Write};

use smplc_ir::Call;

use crate::builder::Builder;
use crate::env::Env;

use crate::compile::Compile;

use super::to_asm;

impl Compile for Call {
    fn compile(self, env: &mut Env, builder: &mut Builder) -> fmt::Result {
        let shift = env.stack_size() + self.args.len() * 8;

        for (n, arg) in self.args.into_iter().rev().enumerate() {
            let value = to_asm(env, builder, arg);

            writeln!(builder, "movss xmm0, {value}")?;
            writeln!(
                builder,
                "movss DWORD [rsp - {}], xmm0",
                env.stack_size() + (n + 1) * 8
            )?;
        }

        writeln!(builder, "sub rsp, {shift}")?;
        writeln!(builder, "call {}", self.id)?;
        writeln!(builder, "add rsp, {shift}")
    }
}
