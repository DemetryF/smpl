use std::fmt::{self, Write};

use middleend::Copy;

use crate::builder::Builder;
use crate::env::Env;

use crate::compile::Compile;

impl Compile for Copy {
    fn compile(self, env: &mut Env, builder: &mut Builder) -> fmt::Result {
        let result = env.add(&self.result.0);

        let value = match self.value {
            middleend::Atom::Id(id) => env.get(&id),
            middleend::Atom::Number(num) => builder.float(num),
        };

        writeln!(builder, "movss xmm0, {value}")?;
        writeln!(builder, "movss {result}, xmm0")
    }
}
