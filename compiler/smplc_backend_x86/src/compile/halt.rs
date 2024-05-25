use std::fmt::{self, Write};

use smplc_lir::Halt;

use crate::builder::Builder;
use crate::env::Env;
use crate::Compile;

impl Compile for Halt {
    fn compile(self, _env: &mut Env, builder: &mut Builder) -> fmt::Result {
        writeln!(
            builder,
            "
mov rbx, 0
mov rax, 1
syscall
            "
        )
    }
}
