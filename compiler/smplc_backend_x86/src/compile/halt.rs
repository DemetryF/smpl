use std::fmt::{self, Write};

use smplc_ir::Halt;

use crate::builder::Builder;
use crate::env::Env;
use crate::Compile;

impl Compile for Halt {
    fn compile(self, _env: &mut Env, builder: &mut Builder) -> fmt::Result {
        writeln!(
            builder,
            "
    mov ebx, 0
    mov eax, 1
    int 0x80
            "
        )
    }
}
