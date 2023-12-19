use std::fmt;

use crate::{Atom, Id};

impl fmt::Display for Atom {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Atom::Id(id) => write!(f, "{id}"),
            Atom::Literal(literal) => write!(f, "{literal}"),
        }
    }
}

impl fmt::Display for Id {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.id)
    }
}
