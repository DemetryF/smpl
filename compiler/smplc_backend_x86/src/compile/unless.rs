use std::fmt::{self, Write};

use smplc_lir::Unless;

use crate::builder::Builder;
use crate::env::Env;

use crate::compile::Compile;

impl Compile for Unless {
    fn compile(self, env: &mut Env, builder: &mut Builder) -> fmt::Result {
        match self.cond {
            smplc_lir::Atom::Id(id) => {
                writeln!(builder, "movss xmm0, {}", env.get(id))?;
                writeln!(builder, "xorpd xmm1, xmm1")?;
                writeln!(builder, "ucomiss xmm0, xmm1")?;
                writeln!(builder, "jz {}", self.label)
            }

            smplc_lir::Atom::Number(num) => {
                if num == 0.0 {
                    writeln!(builder, "jmp {}", self.label)?;
                }

                Ok(())
            }
        }
    }
}
