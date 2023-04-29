use std::fmt::{self, Write};

use middleend::Goto;

use crate::builder::Builder;
use crate::env::Env;

use crate::compile::Compile;

impl Compile for Goto {
    fn compile(self, _env: &mut Env, builder: &mut Builder) -> fmt::Result {
        writeln!(builder, "jmp {}", self.label)
    }
}
