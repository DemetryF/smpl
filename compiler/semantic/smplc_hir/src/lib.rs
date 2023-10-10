pub mod expr;
pub mod statements;

use std::rc::Rc;

use smplc_ast::Pos;
use statements::Statement;

pub use smplc_ast::operators;

pub struct VarInfo {
    pub id: String,
    pub definition_pos: Pos,
}

pub struct FunInfo {
    pub id: String,
    pub definition_pos: Pos,
    pub args: Vec<VarInfo>,
    pub body: Vec<Statement>,
}

pub type VarRef = Rc<VarInfo>;
pub type FunRef = Rc<FunInfo>;

pub struct Block {
    pub statements: Vec<Statement>,
}
