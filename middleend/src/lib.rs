pub use code::{Code, CodeFunction};
pub use error::Error;
pub use instruction::*;

use frontend::ast::{Pos, Statement};
use scopes::Function;
use translator::Translator;

mod code;
mod error;
mod instruction;
mod scopes;
mod translate;
mod translator;

pub fn translate(stmts: Vec<Statement>) -> Result<Code, Vec<Error>> {
    let mut translator = Translator::default();

    let print_id = frontend::ast::Id::new("print".into(), Pos::default());

    translator
        .scopes
        .add_function(
            &print_id,
            Function {
                defined_at: Pos::default(),
                args_count: 1,
            },
        )
        .unwrap();

    let (global, local): (Vec<Statement>, Vec<Statement>) = stmts
        .into_iter()
        .partition(|stmt| matches!(stmt, Statement::Function(_)));

    global
        .into_iter()
        .for_each(|stmt| translator.translate(stmt));

    translator
        .code
        .add_function(CodeFunction::new("main".into(), vec![]));

    local
        .into_iter()
        .for_each(|stmt| translator.translate(stmt));

    if translator.errors.is_empty() {
        Ok(translator.code)
    } else {
        Err(translator.errors)
    }
}
