use std::collections::HashMap;
use std::fmt::{self, Write};

use smplc_ir as ir;

use builder::Builder;
use compile::Compile;
use env::Env;

mod builder;
mod compile;
mod env;

pub fn compile(code: ir::Code) -> Result<String, fmt::Error> {
    let mut builder = Builder::default();

    let constants = code
        .constants
        .into_iter()
        .map(|(id, value)| (id, builder.float(value)))
        .collect::<HashMap<_, _>>();

    writeln!(
        builder,
        "
section .text
global main
extern printf
    "
    )?;

    writeln!(
        builder,
        "
print:
movss xmm0, [rsp+8]
cvtss2sd xmm0, xmm0
lea rdi, [fmt]
mov rax, 1
test rsp, 15
jne print_L1
call printf
jmp print_L0
print_L1:
sub rsp, 8
call printf
add rsp, 8 
print_L0:
ret
        "
    )?;

    for function in code.functions {
        let mut env = Env::new(&constants);

        writeln!(builder, "{}:", function.id)?;

        writeln!(builder, "push rbp")?;
        writeln!(builder, "mov rbp, rsp")?;

        for (index, arg) in function.args.into_iter().enumerate() {
            env.set(arg, -(index as isize + 2));
        }

        let instructions_count = function.instructions.len();

        for (index, instruction) in function.instructions.into_iter().enumerate() {
            if let Some(label) = function.labels.get(&index) {
                writeln!(builder, "{label}:")?;
            }

            instruction.compile(&mut env, &mut builder)?;
        }

        if let Some(label) = function.labels.get(&instructions_count) {
            writeln!(builder, "{label}:")?;
        }
    }

    builder.build()
}
