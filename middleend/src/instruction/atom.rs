use frontend::ast::Literal;

use super::Id;

pub enum Atom {
    Id(String),
    Number(f32),
}

impl std::fmt::Display for Atom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Atom::Id(id) => write!(f, "{id}"),
            Atom::Number(num) => write!(f, "{num}"),
        }
    }
}

impl From<Id> for Atom {
    fn from(value: Id) -> Self {
        Self::Id(value.0)
    }
}

impl From<Literal> for Atom {
    fn from(value: Literal) -> Self {
        let number = match value {
            Literal::Number(number) => number,
            Literal::Bool(b) => b as u32 as f32,
        };

        Self::Number(number)
    }
}
