use smplc_ast as ast;

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

pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
}

impl TryFrom<ast::BinOp> for BinOp {
    type Error = ();

    fn try_from(value: smplc_ast::BinOp) -> Result<Self, Self::Error> {
        match value {
            ast::BinOp::Add => Ok(BinOp::Add),
            ast::BinOp::Sub => Ok(BinOp::Sub),
            ast::BinOp::Mul => Ok(BinOp::Mul),
            ast::BinOp::Div => Ok(BinOp::Div),

            _ => Err(()),
        }
    }
}

pub enum RelOp {
    Eq,
    Ne,
    Le,
    Lt,
    Ge,
    Gt,
}

impl TryFrom<ast::BinOp> for RelOp {
    type Error = ();

    fn try_from(value: ast::BinOp) -> Result<Self, Self::Error> {
        match value {
            ast::BinOp::Ne => Ok(RelOp::Ne),
            ast::BinOp::Eq => Ok(RelOp::Eq),
            ast::BinOp::Ge => Ok(RelOp::Ge),
            ast::BinOp::Gt => Ok(RelOp::Gt),
            ast::BinOp::Le => Ok(RelOp::Le),
            ast::BinOp::Lt => Ok(RelOp::Lt),

            _ => Err(()),
        }
    }
}
