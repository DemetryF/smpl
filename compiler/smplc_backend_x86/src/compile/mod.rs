mod control_flow;
mod sequental;

use std::fmt;

use smplc_lir::{Atom, Number};

use crate::{builder::Builder, env::Env};

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
