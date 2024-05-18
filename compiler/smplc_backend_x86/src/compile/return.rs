use std::fmt::{self, Write};

use smplc_lir::Return;

use crate::builder::Builder;
use crate::compile::{to_asm, Compile};
use crate::env::Env;

impl Compile for Return {
    fn compile(self, env: &mut Env, builder: &mut Builder) -> fmt::Result {
        if let Some(operand) = self.value {
            let operand = to_asm(env, builder, operand);

            writeln!(builder, "movss xmm0, {}", operand)?;
        }

        writeln!(builder, "pop rbp")?;
        writeln!(builder, "ret")
    }
}
