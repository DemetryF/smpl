use crate::{Block, Expr, Id};

#[derive(Debug, PartialEq)]
pub enum Declaration<'source> {
    Function(FunctionDeclaration<'source>),
    Constant(ConstantDeclaration<'source>),
}

#[derive(PartialEq, Debug)]
pub struct FunctionDeclaration<'source> {
    pub id: Id<'source>,
    pub args: Vec<Id<'source>>,
    pub body: Block<'source>,
}

#[derive(Debug, PartialEq)]
pub struct ConstantDeclaration<'source> {
    pub id: Id<'source>,
    pub value: Expr<'source>,
}
