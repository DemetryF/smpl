mod ops;

use smplc_ast as ast;
use smplc_hir::SymbolsTable;

pub use smplc_ast::LiteralType;
pub use smplc_hir::{Atom, FunData, FunId, Literal, Type, VarId};

pub use ops::*;

pub struct THIR<'source> {
    pub symbols: Symbols<'source>,

    pub functions: Vec<Function<'source>>,
    pub constants: Vec<Constant<'source>>,
}

pub struct Symbols<'source> {
    pub functions: SymbolsTable<FunId, FunData<'source>>,
    pub variables: SymbolsTable<VarId, VarData<'source>>,
}

pub struct VarData<'source> {
    pub id: ast::Id<'source>,
    pub ty: Type,
}

pub struct Function<'source> {
    pub id: FunId,
    pub args: Vec<VarId>,
    pub body: Block<'source>,
}

pub struct Constant<'source> {
    pub id: VarId,
    pub ty: Type,
    pub value: Expr<'source>,
}

pub struct Block<'source> {
    pub statements: Vec<Statement<'source>>,
}

pub enum Statement<'source> {
    Expr(ExprStatement<'source>),
    If(IfStatement<'source>),
    Return(ReturnStatement<'source>),
    While(WhileStatement<'source>),
    Break,
    Continue,
}

pub enum ExprStatement<'source> {
    Assign { var: VarId, rhs: Expr<'source> },
    Expr(Expr<'source>),
}

pub struct IfStatement<'source> {
    pub cond: Expr<'source>,
    pub body: Block<'source>,
    pub else_body: Option<Block<'source>>,
}

pub struct ReturnStatement<'source> {
    pub value: Option<Expr<'source>>,
}

pub struct WhileStatement<'source> {
    pub cond: Expr<'source>,
    pub body: Block<'source>,
}

pub enum Expr<'source> {
    Binary {
        lhs: Box<Self>,
        op: BinOp,
        rhs: Box<Self>,
    },
    Unary {
        op: UnOp,
        rhs: Box<Self>,
    },
    Call {
        fun: FunId,
        args: Vec<Self>,
    },
    Atom(Atom<'source>),
}
