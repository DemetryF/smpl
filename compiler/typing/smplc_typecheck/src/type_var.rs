use std::fmt;

use smplc_hir::Type;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TypeVar {
    Type(Type),
    /// Real, Integer
    Scalar,
    /// Scalar, Complex
    Number,
    /// Vec3, Vec4
    Vec34,
    /// Vec2, Vec3, Vec4
    Vec,
    /// Number, Vec
    Linear,
    Unknown,
    None,
}

impl TypeVar {
    pub fn is_vec(self) -> bool {
        matches!(
            self,
            Self::Vec34
                | Self::Vec
                | Self::Type(Type::Vec2)
                | Self::Type(Type::Vec3)
                | Self::Type(Type::Vec4)
        )
    }

    pub fn is_scalar(self) -> bool {
        matches!(
            self,
            Self::Scalar | Self::Type(Type::Real) | Self::Type(Type::Int)
        )
    }

    pub fn is_number(self) -> bool {
        self.is_scalar() || matches!(self, Self::Number | Self::Type(Type::Complex))
    }

    pub fn is_linear(self) -> bool {
        self.is_vec() || self.is_number()
    }

    pub fn max(a: Self, b: Self) -> Result<Self, (Self, Self)> {
        match (a, b) {
            (a, b) if a == b => Ok(a),

            (Self::Unknown, res) | (res, Self::Unknown) => Ok(res),

            (Self::Type(Type::Complex), Self::Type(Type::Real))
            | (Self::Type(Type::Real), Self::Type(Type::Complex)) => Ok(Type::Complex.into()),

            (Self::Linear, ty) | (ty, Self::Linear) if ty.is_linear() => Ok(ty),
            (Self::Number, ty) | (ty, Self::Number) if ty.is_number() => Ok(ty),
            (Self::Scalar, ty) | (ty, Self::Scalar) if ty.is_scalar() => Ok(ty),
            (Self::Vec34, ty @ Self::Type(Type::Vec3 | Type::Vec4))
            | (ty @ Self::Type(Type::Vec3 | Type::Vec4), Self::Vec34) => Ok(ty),
            (Self::Vec, ty) | (ty, Self::Vec) if ty.is_vec() => Ok(ty),

            _ => Err((a, b)),
        }
    }
}

impl fmt::Display for TypeVar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TypeVar::Type(ty) => write!(f, "{ty}"),
            TypeVar::Scalar => write!(f, "AmbiguousScalar"),
            TypeVar::Number => write!(f, "AmbiguousNumber"),
            TypeVar::Vec34 => write!(f, "AmbiguousVec{{3,4}}"),
            TypeVar::Vec => write!(f, "AmbiguousVec"),
            TypeVar::Linear => write!(f, "AmbiguousLinear"),
            TypeVar::Unknown => write!(f, "Unknown"),
            TypeVar::None => write!(f, "None"),
        }
    }
}

impl From<Type> for TypeVar {
    fn from(value: Type) -> Self {
        Self::Type(value)
    }
}
