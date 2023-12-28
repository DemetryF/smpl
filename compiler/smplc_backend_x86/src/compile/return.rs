use std::fmt::{self, Write};

use smplc_ir::Return;

use crate::builder::Builder;
use crate::compile::Compile;
use crate::env::Env;

impl Compile for Return {
    fn compile(self, env: &mut Env, builder: &mut Builder) -> fmt::Result {
        if let Some(operand) = self.value {
            let operand = match operand {
                smplc_ir::Atom::Id(id) => env.get(id),
                smplc_ir::Atom::Number(num) => builder.float(num),
            };

            writeln!(builder, "movss xmm0, {}", operand)?;
        }

        writeln!(builder, "pop rbp")?;
        writeln!(builder, "ret")
    }
}
