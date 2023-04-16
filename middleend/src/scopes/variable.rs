use frontend::ast::Pos;

use crate::instruction::Id;

#[derive(Clone)]
pub struct Variable {
    pub defined_at: Pos,
    pub id: Id,
}
