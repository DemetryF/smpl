use std::collections::HashMap;
use std::fmt::{self, Write};

use ir::{Number, NumberType};
use smplc_lir as ir;

use builder::Builder;
use compile::Compile;
use env::Env;

mod builder;
mod compile;
mod env;

pub fn compile(code: ir::Code, types: HashMap<ir::Id, NumberType>) -> Result<String, fmt::Error> {
    let mut builder = Builder::default();

    let constants = code
        .constants
        .into_iter()
        .map(|(id, value)| match value {
            Number::Real(value) => (id, builder.float(value)),
            Number::Int(value) => (id, value.to_string()),
        })
        .collect::<HashMap<_, _>>();

    writeln!(
        builder,
        "\
section .text
global main
extern printf
    "
    )?;

    writeln!(
        builder,
        "\
printr:
        movss xmm0, [rsp+8]
        cvtss2sd xmm0, xmm0
        lea rdi, [fmtr]
        mov rax, 1
        test rsp, 15
        jne printr_L1
        call printf
        jmp printr_L0
    printr_L1:
        sub rsp, 8
        call printf
        add rsp, 8 
    printr_L0:
        ret\
        "
    )?;

    writeln!(
        builder,
        "\
printi:
    mov rsi, [rsp+8]
    lea rdi, [fmti]

    test rsp, 15
    jne printi_L1
    call printf
    jmp printi_L0
  printi_L1:
    sub rsp, 8
    call printf
    add rsp, 8 
  printi_L0:
    ret\
        "
    )?;

    writeln!(
        builder,
        "\
printb:
    cmp dword[rsp+8], 0
    je printb_L0
    lea rdi, [fmttrue]
    jmp printb_L1
  printb_L0:
    lea rdi, [fmtfalse]
  printb_L1:
    
    test rsp, 15
    jne printb_L2
    call printf
    jmp printb_L3
  printb_L2:
    sub rsp, 8
    call printf
    add rsp, 8 
  printb_L3:\
    ret
    "
    )?;

    for function in code.functions {
        let mut env = Env::new(&constants, &types);

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
