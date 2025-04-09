mod control_flow;
mod sequental;

use std::fmt;

use comet_ir as lir;

use crate::{
    builder::Builder,
    env::{Env, Operand},
};

pub trait Compile {
    fn compile(self, env: &mut Env, builder: &mut Builder) -> fmt::Result;
}

pub fn atom(env: &mut Env, builder: &mut Builder, atom: lir::Atom) -> Operand {
    match atom {
        lir::Atom::Id(id) => env.get(id),
        lir::Atom::Value(v) => value(builder, v),
    }
}

pub fn value(builder: &mut Builder, value: lir::Value) -> Operand {
    match value {
        lir::Value::Real(value) => Operand::Address(builder.constant([value])),
        lir::Value::Int(value) => Operand::Number(value),
        lir::Value::F32x2(value) => Operand::Address(builder.constant([value[0], value[1]])),

        lir::Value::F32x3(value) => {
            Operand::Address(builder.constant([value[0], value[1], value[2]]))
        }

        lir::Value::F32x4(value) => {
            Operand::Address(builder.constant([value[0], value[1], value[2], value[3]]))
        }
    }
}
