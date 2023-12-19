use smplc_ast::Pos;

#[derive(Clone)]
pub struct Function {
    pub defined_at: Pos,
    pub args_count: usize,
}
