mod display;

use smplc_ast::{self as ast, Type};

#[derive(PartialEq, Eq)]
pub enum BinOp {
    Rel(RelOp, NumberType),
    Arithm(ArithmOp, NumberType),
    Or,
    And,
}

#[derive(PartialEq, Eq)]
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

#[derive(PartialEq, Eq)]
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

#[derive(PartialEq, Eq)]
pub enum UnOp {
    Neg(NumberType),
    Not,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum NumberType {
    Real,
    Int,
}

impl NumberType {
    pub fn for_ir(ty: Type) -> Self {
        match ty {
            Type::Real => Self::Real,
            Type::Bool | Type::Int => Self::Int,
        }
    }
}

impl TryFrom<Type> for NumberType {
    type Error = ();

    fn try_from(value: Type) -> Result<Self, Self::Error> {
        match value {
            Type::Real => Ok(Self::Real),
            Type::Int => Ok(Self::Int),
            Type::Bool => Err(()),
        }
    }
}

impl From<NumberType> for Type {
    fn from(value: NumberType) -> Type {
        match value {
            NumberType::Real => Type::Real,
            NumberType::Int => Type::Int,
        }
    }
}
