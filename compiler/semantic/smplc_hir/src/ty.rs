use std::fmt;

use comet_ir as lir;
use smplc_ast::LiteralType;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum Type {
    Real,
    Int,
    Bool,
    Vec2,
    Vec3,
    Vec4,
    Complex,
}

impl Into<lir::Type> for Type {
    fn into(self) -> lir::Type {
        match self {
            Type::Real => lir::Type::Real,
            Type::Int | Type::Bool => lir::Type::Int,
            Type::Complex | Type::Vec2 => lir::Type::F32x2,
            Type::Vec3 => lir::Type::F32x3,
            Type::Vec4 => lir::Type::F32x4,
        }
    }
}

impl TryFrom<&str> for Type {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "real" => Ok(Self::Real),
            "int" => Ok(Self::Int),
            "bool" => Ok(Self::Bool),
            "vec2" => Ok(Self::Vec2),
            "vec3" => Ok(Self::Vec3),
            "vec4" => Ok(Self::Vec4),
            "complex" => Ok(Self::Complex),

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
            LiteralType::Complex => Self::Complex,
        }
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Type::Real => write!(f, "real"),
            Type::Int => write!(f, "int"),
            Type::Bool => write!(f, "bool"),
            Type::Vec2 => write!(f, "vec2"),
            Type::Vec3 => write!(f, "vec3"),
            Type::Vec4 => write!(f, "vec4"),
            Type::Complex => write!(f, "complex"),
        }
    }
}
