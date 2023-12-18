mod block;
mod expr;
mod id;
mod operators;
mod statement;

pub use self::statement::{
    DeclareStatement, ExprStatement, FunctionStatement, IfStatement, ReturnStatement, Statement,
    WhileStatement,
};

pub use self::{
    block::Block,
    expr::{Atom, Call, Expr},
    id::Id,
    operators::{BinOp, UnOp},
};

pub use smplc_lexer::{Literal, Pos};

use crate::{ParseError, TokenStream};

pub trait Collect: Sized {
    fn collect(token_stream: &mut TokenStream) -> Result<Self, ParseError>;
}
