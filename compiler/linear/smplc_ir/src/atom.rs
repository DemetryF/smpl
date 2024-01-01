use smplc_ast as ast;

use crate::Id;

#[derive(Clone, PartialEq)]
pub enum Atom {
    Id(Id),
    Number(f32),
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
