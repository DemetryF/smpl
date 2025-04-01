use std::{
    collections::HashMap,
    fmt::{self, Write},
};

use smplc_lir as ir;

use builder::Builder;
use compile::{value, Compile};
use env::Env;

mod builder;
mod compile;
mod env;

pub fn compile(lir: ir::LIR) -> Result<String, fmt::Error> {
    let mut builder = Builder::default();

    let constants = lir
        .constants
        .into_iter()
        .map(|(id, v)| (id, value(&mut builder, v)))
        .collect::<HashMap<_, _>>();

    writeln!(
        builder,
        "\
section .text
global main
extern printf"
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
    ret"
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
    ret"
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
printb_L3:
    ret"
    )?;

    for (id, function) in lir.functions {
        let mut env = Env::new(
            &constants,
            &lir.labels,
            &function.code.phis,
            &lir.function_names,
        );

        writeln!(builder, "{}:", lir.function_names[&id])?;
        writeln!(builder, "push rbp")?;
        writeln!(builder, "mov rbp, rsp")?;

        for (index, arg) in function.args.into_iter().enumerate() {
            env.set(arg, -(index as isize + 2));
        }

        for block in function.code.blocks {
            if let Some(label) = block.label {
                write!(builder, "{}:", lir.labels[&label])?;
            }

            for instr in block.instructions {
                write!(builder, "; {}", &instr)?;

                instr.compile(&mut env, &mut builder)?;
            }

            if let Some(end) = block.end {
                write!(builder, "; {}", &end)?;

                end.compile(&mut env, &mut builder)?;
            }
        }
    }

    builder.build()
}
