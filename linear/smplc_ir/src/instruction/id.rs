#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Id(pub String);

impl std::fmt::Display for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<'source> From<smplc_ast::Id<'source>> for Id {
    fn from(value: smplc_ast::Id) -> Self {
        Self(value.id.into())
    }
}
