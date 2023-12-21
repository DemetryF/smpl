use smplc_ast::Pos;

use smplc_ir::Id;

#[derive(Clone)]
pub struct Variable {
    pub defined_at: Pos,
    pub id: Id,
}
