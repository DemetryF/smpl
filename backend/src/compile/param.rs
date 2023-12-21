use std::fmt::{self, Write};

use smplc_ir::Param;

use crate::{builder::Builder, env::Env};

use super::Compile;

impl Compile for Param {
    fn compile(self, env: &mut Env, builder: &mut Builder) -> fmt::Result {
        let value = match self.value {
            smplc_ir::Atom::Number(num) => builder.float(num),
            smplc_ir::Atom::Id(id) => env.get(&id),
        };

        env.variables_count += 1;

        writeln!(builder, "movss xmm0, {value}")?;
        writeln!(
            builder,
            "movss DWORD [rsp - {}], xmm0",
            env.variables_count * 8
        )
    }
}
