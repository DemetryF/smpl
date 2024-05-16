mod assign;
mod call;
mod goto;
mod halt;
mod r#if;
mod r#return;
mod unless;

use std::fmt::{self, Write};

use smplc_ir::{Atom, Instruction};

use crate::{builder::Builder, env::Env};

pub trait Compile {
    fn compile(self, env: &mut Env, builder: &mut Builder) -> fmt::Result;
}

impl Compile for Instruction {
    fn compile(self, env: &mut Env, builder: &mut Builder) -> fmt::Result {
        writeln!(builder, "; {}", self)?;

        match self {
            Instruction::Assign(a) => a.compile(env, builder),
            Instruction::If(a) => a.compile(env, builder),
            Instruction::Unless(a) => a.compile(env, builder),
            Instruction::Goto(a) => a.compile(env, builder),
            Instruction::Call(a) => a.compile(env, builder),
            Instruction::Return(a) => a.compile(env, builder),
            Instruction::Halt(a) => a.compile(env, builder),
        }
    }
}

pub fn to_asm(env: &mut Env, builder: &mut Builder, atom: Atom) -> String {
    match atom {
        Atom::Id(id) => env.get(id),
        Atom::Number(value) => builder.float(value),
    }
}
