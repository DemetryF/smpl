mod binary;
mod call;
mod copy;
mod goto;
mod halt;
mod r#if;
mod r#return;
mod unary;
mod unless;

use std::fmt::{self, Write};

use smplc_ir::Instruction;

use crate::{builder::Builder, env::Env};

pub trait Compile {
    fn compile(self, env: &mut Env, builder: &mut Builder) -> fmt::Result;
}

impl Compile for Instruction {
    fn compile(self, env: &mut Env, builder: &mut Builder) -> fmt::Result {
        writeln!(builder, "; {}", self)?;

        match self {
            Instruction::Binary(a) => a.compile(env, builder),
            Instruction::Unary(a) => a.compile(env, builder),
            Instruction::Copy(a) => a.compile(env, builder),
            Instruction::If(a) => a.compile(env, builder),
            Instruction::Unless(a) => a.compile(env, builder),
            Instruction::Goto(a) => a.compile(env, builder),
            Instruction::Call(a) => a.compile(env, builder),
            Instruction::Return(a) => a.compile(env, builder),
            Instruction::Halt(a) => a.compile(env, builder),
        }
    }
}
