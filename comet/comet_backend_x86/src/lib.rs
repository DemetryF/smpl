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

    writeln!(builder, include_str!("std.nasm"))?;

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
