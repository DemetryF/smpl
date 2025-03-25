use std::fmt;

use smplc_ast::LiteralType;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum Type {
    Real,
    Int,
    Bool,
}

impl TryFrom<&str> for Type {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "real" => Ok(Self::Real),
            "int" => Ok(Self::Int),
            "bool" => Ok(Self::Bool),

            _ => Err(()),
        }
    }
}

impl From<LiteralType> for Type {
    fn from(value: LiteralType) -> Self {
        match value {
            LiteralType::Real => Self::Real,
            LiteralType::Int => Self::Int,
            LiteralType::Bool => Self::Bool,
        }
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Type::Real => write!(f, "real"),
            Type::Int => write!(f, "int"),
            Type::Bool => write!(f, "bool"),
        }
    }
}
