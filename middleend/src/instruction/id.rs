use derive_more::Constructor;

#[derive(Constructor, Clone)]
pub struct Id(pub String);

impl std::fmt::Display for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<frontend::ast::Id> for Id {
    fn from(value: frontend::ast::Id) -> Self {
        Self(value.id)
    }
}
