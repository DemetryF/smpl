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

pub enum BinOp {
    Arithm(ArithmOp, NumberType),
    Rel(RelOp, NumberType),
    Or,
    And,
}

pub enum ArithmOp {
    Add,
    Sub,
    Mul,
    Div,
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

pub enum RelOp {
    Eq,
    Ne,
    Gt,
    Ge,
    Lt,
    Le,
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

#[derive(Clone, Copy)]
pub enum NumberType {
    Real,
    Int,
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

pub enum UnOp {
    Neg(NumberType),
    Not,
}
