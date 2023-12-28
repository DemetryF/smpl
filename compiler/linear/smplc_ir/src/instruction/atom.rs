use smplc_ast as ast;

use crate::Id;

#[derive(Clone, PartialEq)]
pub enum Atom {
    Id(Id),
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

impl From<ast::Literal> for Atom {
    fn from(value: ast::Literal) -> Self {
        let number = match value {
            ast::Literal::Number(number) => number,
            ast::Literal::Bool(b) => b as u32 as f32,
        };

        Self::Number(number)
    }
}
