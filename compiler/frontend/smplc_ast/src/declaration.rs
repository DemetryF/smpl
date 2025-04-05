use crate::{Block, Expr, Id, Spanned};

#[derive(Debug, PartialEq)]
pub enum Declaration<'source> {
    Function(FunctionDeclaration<'source>),
    Constant(ConstantDeclaration<'source>),
}

#[derive(PartialEq, Debug)]
pub struct FunctionDeclaration<'source> {
    pub id: Id<'source>,
    pub args: Vec<FunctionArg<'source>>,
    pub ret_ty: Option<Id<'source>>,
    pub body: Block<'source>,
}

#[derive(PartialEq, Debug)]
pub struct FunctionArg<'source> {
    pub id: Id<'source>,
    pub ty: Id<'source>,
}

#[derive(Debug, PartialEq)]
pub struct ConstantDeclaration<'source> {
    pub id: Id<'source>,
    pub ty: Id<'source>,
    pub value: Spanned<Expr<'source>>,
}
