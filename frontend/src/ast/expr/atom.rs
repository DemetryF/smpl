use crate::ast::{Id, Literal};

#[derive(PartialEq, Debug)]
pub enum Atom {
    Id(Id),
    Literal(Literal),
}

impl std::fmt::Display for Atom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Atom::Id(id) => write!(f, "{id}"),
            Atom::Literal(literal) => write!(f, "{literal}"),
        }
    }
}
