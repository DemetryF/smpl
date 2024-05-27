use std::rc::Rc;

pub use code::{Code, CodeFunction, Number};
pub use instructions::*;

mod code;
mod display;
mod instructions;

#[derive(Clone)]
pub struct Label(Rc<str>);

impl Label {
    pub fn new(name: impl Into<Rc<str>>) -> Self {
        Self(name.into())
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Id(usize);

impl From<usize> for Id {
    fn from(value: usize) -> Self {
        Self(value)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FunctionId(pub Rc<str>);

#[derive(Clone, Copy, PartialEq)]
pub enum Atom {
    Real(f32),
    Int(i32),
    Id(Id),
}
