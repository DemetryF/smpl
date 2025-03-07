mod control_flow;
mod sequental;

use std::fmt;

use smplc_lir::{Atom, Number, Type};

use crate::builder::Builder;
use crate::env::Env;

pub trait Compile {
    fn compile(self, env: &mut Env, builder: &mut Builder) -> fmt::Result;
}

pub fn to_asm(env: &mut Env, builder: &mut Builder, atom: Atom) -> String {
    match atom {
        Atom::Id(id) => env.get(id),
        Atom::Number(Number::Real(value)) => builder.float(value),
        Atom::Number(Number::Int(value)) => value.to_string(),
    }
}

// replace it with ty
pub fn to_asm_with_ty(env: &Env, builder: &mut Builder, atom: Atom) -> (String, Type) {
    match atom {
        Atom::Id(id) => (env.get(id), env.ty(id)),
        Atom::Number(Number::Real(value)) => (builder.float(value), Type::Real),
        Atom::Number(Number::Int(value)) => (value.to_string(), Type::Int),
    }
}
