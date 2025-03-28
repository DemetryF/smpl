use std::fmt;

use smplc_ast as ast;
use smplc_hir::SymbolsTable;

pub use smplc_hir::{Atom, FunData, FunId, Type, VarId};

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

#[derive(PartialEq, Eq)]
pub enum BinOp {
    Arithm(ArithmOp, NumberType),
    Rel(RelOp, NumberType),
    Or,
    And,
}

#[derive(PartialEq, Eq)]
pub enum ArithmOp {
    Add,
    Sub,
    Mul,
    Div,
}

impl fmt::Display for ArithmOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ArithmOp::Add => write!(f, "+"),
            ArithmOp::Sub => write!(f, "-"),
            ArithmOp::Mul => write!(f, "*"),
            ArithmOp::Div => write!(f, "/"),
        }
    }
}

impl TryFrom<ast::BinOp> for ArithmOp {
    type Error = ();

    fn try_from(value: ast::BinOp) -> Result<Self, Self::Error> {
        match value {
            ast::BinOp::Add => Ok(Self::Add),
            ast::BinOp::Sub => Ok(Self::Sub),
            ast::BinOp::Mul => Ok(Self::Mul),
            ast::BinOp::Div => Ok(Self::Div),
            _ => Err(()),
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum RelOp {
    Eq,
    Ne,
    Gt,
    Ge,
    Lt,
    Le,
}

impl fmt::Display for RelOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RelOp::Eq => write!(f, "=="),
            RelOp::Ne => write!(f, "!="),
            RelOp::Gt => write!(f, ">"),
            RelOp::Ge => write!(f, ">="),
            RelOp::Lt => write!(f, "<"),
            RelOp::Le => write!(f, "<="),
        }
    }
}

impl TryFrom<ast::BinOp> for RelOp {
    type Error = ();

    fn try_from(value: ast::BinOp) -> Result<Self, Self::Error> {
        match value {
            ast::BinOp::Ne => Ok(Self::Ne),
            ast::BinOp::Eq => Ok(Self::Eq),
            ast::BinOp::Ge => Ok(Self::Ge),
            ast::BinOp::Gt => Ok(Self::Gt),
            ast::BinOp::Le => Ok(Self::Le),
            ast::BinOp::Lt => Ok(Self::Lt),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum NumberType {
    Real,
    Int,
}

impl NumberType {
    pub fn for_ir(ty: Type) -> Self {
        match ty {
            Type::Real => Self::Real,
            Type::Int | Type::Bool => Self::Int,
        }
    }
}

impl fmt::Display for NumberType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NumberType::Real => write!(f, "real"),
            NumberType::Int => write!(f, "int"),
        }
    }
}

impl Into<Type> for NumberType {
    fn into(self) -> Type {
        match self {
            NumberType::Real => Type::Real,
            NumberType::Int => Type::Int,
        }
    }
}

impl TryInto<NumberType> for Type {
    type Error = ();

    fn try_into(self) -> Result<NumberType, Self::Error> {
        match self {
            Type::Real => Ok(NumberType::Real),
            Type::Int => Ok(NumberType::Int),
            Type::Bool => Err(()),
        }
    }
}

#[derive(PartialEq, Eq)]
pub enum UnOp {
    Neg(NumberType),
    Not,
}
