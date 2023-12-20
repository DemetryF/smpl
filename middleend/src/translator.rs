use smplc_ast::Statement;

use crate::{instruction::Id, scopes::Scopes, translate::Translate, Code, Error};

#[derive(Default)]
pub struct Translator<'source> {
    pub scopes: Scopes<'source>,
    pub errors: Vec<Error<'source>>,

    pub code: Code,

    pub ifs_count: usize,
    pub whiles_count: usize,
}

impl<'source> Translator<'source> {
    pub fn create_temp_variable(&mut self) -> Id {
        let name = format!("${}", self.scopes.inc_variables_counter());

        Id::new(name)
    }

    pub fn translate(&mut self, stmt: Statement<'source>) {
        if let Err(error) = stmt.translate(self) {
            self.errors.push(error);
        }
    }
}
