use std::fmt::{self, Write};

use smplc_ir::Call;

use crate::builder::Builder;
use crate::env::Env;

use crate::compile::Compile;

impl Compile for Call {
    fn compile(self, env: &mut Env, builder: &mut Builder) -> fmt::Result {
        writeln!(builder, "sub rsp, {}", env.size())?;
        writeln!(builder, "call {}", self.id)?;
        writeln!(builder, "add rsp, {}", env.size())?;

        env.variables_count -= builder.function_arg_sizes[&self.id] / 8;

        if let Some(result) = self.result {
            let result_ptr = env.add(result);

            writeln!(builder, "movss {result_ptr}, xmm0")?;
        }

        Ok(())
    }
}
