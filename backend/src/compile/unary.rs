use std::fmt::{self, Write};

use smplc_ir::{UnOp, Unary};

use crate::builder::Builder;
use crate::env::Env;

use crate::compile::Compile;

impl Compile for Unary {
    fn compile(self, env: &mut Env, builder: &mut Builder) -> fmt::Result {
        let result = env.add(&self.result.0);

        match self.rhs {
            smplc_ir::Atom::Id(id) => {
                let operand = env.get(&id);

                match self.op {
                    UnOp::Not => {
                        writeln!(builder, "xorps xmm0, xmm0")?;
                        writeln!(builder, "ucomiss {operand}, xmm0")?;
                        writeln!(builder, "sete ah")?;
                        writeln!(builder, "cvtsi2ss xmm0, ah")?;
                        writeln!(builder, "movss {result}, xmm0")
                    }
                    UnOp::Neg => {
                        writeln!(builder, "pxor xmm0, xmm0")?;
                        writeln!(builder, "subss xmm0, {operand}")?;
                        writeln!(builder, "movss {result}, xmm0")
                    }
                }
            }
            smplc_ir::Atom::Number(mut num) => {
                num = match self.op {
                    UnOp::Not => -num,
                    UnOp::Neg => (num == 0.0) as u32 as f32,
                };

                let result = builder.float(num);

                writeln!(builder, "movss xmm0, {result}")?;
                writeln!(builder, "movss {result}, xmm0")?;

                Ok(())
            }
        }
    }
}
