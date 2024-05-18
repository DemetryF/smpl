pub use smplc_ast::{BinOp, UnOp};

pub use code::{Code, CodeFunction};
pub use instructions::*;

mod code;
mod display;
mod instructions;

#[derive(Clone)]
pub struct Label(pub String);

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Id(usize);

impl From<usize> for Id {
    fn from(value: usize) -> Self {
        Self(value)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct FunctionId(pub String);

#[derive(Clone, PartialEq)]
pub enum Atom {
    Id(Id),
    Number(f32),
}
