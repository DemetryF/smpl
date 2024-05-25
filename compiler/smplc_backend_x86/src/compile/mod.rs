mod assign;
mod call;
mod goto;
mod halt;
mod r#if;
mod r#return;

use std::fmt::{self, Write};

use smplc_lir::{Atom, Instruction, NumberType};

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
        Atom::Real(value) => builder.float(value),
        Atom::Int(value) => value.to_string(),
    }
}

pub fn to_asm_with_ty(env: &Env, builder: &mut Builder, atom: Atom) -> (String, NumberType) {
    match atom {
        Atom::Id(id) => (env.get(id), env.ty(id)),
        Atom::Real(value) => (builder.float(value), NumberType::Real),
        Atom::Int(value) => (value.to_string(), NumberType::Int),
    }
}
