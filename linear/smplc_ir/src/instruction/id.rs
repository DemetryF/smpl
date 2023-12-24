use std::fmt;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Id(usize);

impl fmt::Display for Id {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "${}", self.0)
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Default)]
pub struct FunctionId(pub String);

impl fmt::Display for FunctionId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<&str> for FunctionId {
    fn from(value: &str) -> Self {
        Self(value.into())
    }
}
