mod block;
mod expr;
mod id;
mod operators;
mod statement;

pub use statement::{
    DeclareStatement, ExprStatement, FunctionStatement, IfStatement, ReturnStatement, Statement,
    WhileStatement,
};

pub use self::{
    block::Block,
    expr::{Atom, Expr},
    id::Id,
    operators::{BinOp, UnOp},
};

use crate::{Error, TokenStream};

pub trait Collect: Sized {
    fn collect(token_stream: &mut TokenStream) -> Result<Self, Error>;
}
