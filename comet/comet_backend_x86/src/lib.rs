use std::{
    collections::HashMap,
    fmt::{self, Write},
};

use comet_ir::{self as ir, ControlFlowDisplay};

use builder::Builder;
use compile::{value, Compile};
use env::Env;

mod builder;
mod compile;
mod env;

const STACK_ALIGN: isize = 16;

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
    movss    xmm0, [rsp+8]
    cvtss2sd xmm0, xmm0
    lea      rdi, [fmtr]
    mov      rax, 1
    test     rsp, 15
    jne      printr_L1
    call     printf
    ret
printr_L1:
    sub      rsp, 8
    call     printf
    add      rsp, 8
    ret

printi:
    mov  rsi, [rsp+24]
    lea  rdi, [fmti]

    test rsp, 15
    jne  printi_L1
    call printf
    ret
printi_L1:
    sub  rsp, 8
    call printf
    add  rsp, 8
    ret

printb:
    cmp  dword[rsp+8], 0
    je   printb_L0
    lea  rdi, [fmttrue]
    jmp  printb_L1
printb_L0:
    lea  rdi, [fmtfalse]
printb_L1:
    test rsp, 15
    jne  printb_L2
    call printf
    ret
printb_L2:
    sub  rsp, 8
    call printf
    add  rsp, 8
    ret
"
    )?;

    writeln!(
        builder,
        "\
vec2:                           ; vec2(x = [rsp+16], y = [rsp+32])
    movss    xmm0, [rsp+8]              ; xmm0 = [?, ?, ?, x]
    movss    xmm1, [rsp+24]             ; xmm0 = [?, ?, ?, y]
    unpcklps xmm0, xmm1                 ; xmm0 = [?, ?, y, x]
    ret

vec3:                           ; vec3(x = [rsp+16], y = [rsp+32], z=[rsp+48])
    movss    xmm0, [rsp+8]              ; xmm0 = [?, ?, ?, x]
    unpcklps xmm0, [rsp+40]             ; xmm0 = [?, ?, z, x]
    movss    xmm1, [rsp+24]             ; xmm1 = [?, ?, ?, y]
    unpcklps xmm0, xmm1                 ; xmm0 = [?, z, y, x]
    ret

vec4:                           ; vec4(x = [rsp+16], y = [rsp+32], z = [rsp+48], w = [rsp+64])
    movss    xmm0, [rsp+8]              ; xmm0 = [?, ?, ?, x]
    unpcklps xmm0, [rsp+40]             ; xmm0 = [?, ?, z, x]
    movss    xmm1, [rsp+24]             ; xmm1 = [?, ?, ?, y]
    unpcklps xmm1, [rsp+56]             ; xmm1 = [?, ?, w, y]
    unpcklps xmm0, xmm1                 ; xmm0 = [w, z, y, x]
    ret

printvec2:
    movups   xmm0, [rsp+8]
    movaps   xmm1, xmm0
    shufps   xmm1, xmm1, 0b00_00_00_01
    cvtss2sd xmm0, xmm0
    cvtss2sd xmm1, xmm1
    lea      rdi, [fmtvec2]
    mov      rax, 2
    test     rsp, 15
    jne      printvec2_L1
    call     printf
    ret
printvec2_L1:
    sub      rsp, 8
    call     printf
    add      rsp, 8
    ret

printvec3:
    movaps   xmm0, [rsp+8]
    movaps   xmm1, xmm0
    shufps   xmm1, xmm1, 0b00_00_00_01
    movaps   xmm2, xmm0
    shufps   xmm2, xmm0, 0b00_00_00_10
    cvtss2sd xmm0, xmm0
    cvtss2sd xmm1, xmm1
    cvtss2sd xmm2, xmm2
    lea      rdi, [fmtvec3]
    mov      rax, 3
    test     rsp, 15
    jne      printvec3_L1
    call     printf
    ret
printvec3_L1:
    sub      rsp, 8
    call     printf
    add      rsp, 8
    ret

printvec4:
    movaps   xmm0, [rsp+8]
    movaps   xmm1, xmm0
    shufps   xmm1, xmm0, 0b00_00_00_01
    movaps   xmm2, xmm0
    shufps   xmm2, xmm0, 0b00_00_00_10
    movaps   xmm3, xmm0
    shufps   xmm3, xmm0, 0b00_00_00_11
    cvtss2sd xmm0, xmm0
    cvtss2sd xmm1, xmm1
    cvtss2sd xmm2, xmm2
    cvtss2sd xmm3, xmm3
    lea      rdi, [fmtvec4]
    mov      rax, 4
    test     rsp, 15
    jne      printvec4_L1
    call     printf
    ret
printvec4_L1:
    sub      rsp, 8
    call     printf
    add      rsp, 8
    ret

printc:
    movups   xmm0, [rsp+8]
    movaps   xmm1, xmm0
    shufps   xmm1, xmm1, 0b00_00_00_01
    cvtss2sd xmm0, xmm0
    cvtss2sd xmm1, xmm1
    lea      rdi, [fmtc]
    mov      rax, 2

    ucomisd  xmm1, [zero]
    jne      printc_L0
    lea      rdi, [fmtr]
    mov      rax, 1
printc_L0:

    test     rsp, 15
    jne      printc_L1
    call     printf
    ret
printc_L1:
    sub      rsp, 8
    call     printf
    add      rsp, 8
    ret
    "
    )?;

    for (id, function) in lir.bodies {
        let mut env = Env::new(&constants, &lir.labels, &function.code.phis);

        writeln!(builder, "{id}:")?;
        writeln!(builder, "push rbp")?;
        writeln!(builder, "mov rbp, rsp")?;

        for (index, arg) in function.args.into_iter().enumerate() {
            env.set(arg, -(index as isize + 1));
        }

        for block in function.code.blocks {
            if let Some(label) = block.label {
                writeln!(builder, "{}:", lir.labels[&label])?;
            }

            for instr in block.instructions {
                writeln!(builder, "; {}", &instr)?;

                instr.compile(&mut env, &mut builder)?;
            }

            if let Some(end) = block.end {
                writeln!(builder, "; {}", ControlFlowDisplay(&env.labels, end))?;

                end.compile(&mut env, &mut builder)?;
            }
        }
    }

    builder.build()
}
