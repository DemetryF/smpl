pub use code::Code;
pub use error::Error;

use frontend::ast::Statement;
use instruction::Label;
use translator::Translator;

mod code;
mod error;
mod instruction;
mod scopes;
mod translate;
mod translator;

pub fn translate(stmts: Vec<Statement>) -> Result<Code, Vec<Error>> {
    let mut translator = Translator::default();

    let (global, local) = split_stmts(stmts);

    global
        .into_iter()
        .for_each(|stmt| translator.translate(stmt));

    translator.code.add_label(Label("main".into()));

    local
        .into_iter()
        .for_each(|stmt| translator.translate(stmt));

    if translator.errors.is_empty() {
        Ok(translator.code)
    } else {
        Err(translator.errors)
    }
}

fn split_stmts(stmts: Vec<Statement>) -> (Vec<Statement>, Vec<Statement>) {
    let mut global_stmts = Vec::new();
    let mut local_stmts = Vec::new();

    for stmt in stmts {
        match stmt {
            Statement::Function(_) => global_stmts.push(stmt),
            stmt => local_stmts.push(stmt),
        }
    }

    (global_stmts, local_stmts)
}
