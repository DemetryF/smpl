pub use code::{Code, CodeFunction, Number};
pub use instructions::*;

mod code;
mod display;
mod instructions;

#[derive(Clone)]
pub struct Label(pub String);

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Id(usize);

impl From<usize> for Id {
    fn from(value: usize) -> Self {
        Self(value)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct FunctionId(pub String);

#[derive(Clone, Copy, PartialEq)]
pub enum Atom {
    Real(f32),
    Int(i32),
    Id(Id),
}
