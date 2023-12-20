use std::collections::HashMap;

use crate::error::Error;

use super::Variable;

#[derive(Default)]
pub struct Scope<'source> {
    pub variables: HashMap<&'source str, Variable>,
}

impl<'source> Scope<'source> {
    pub fn get(&self, id: &smplc_ast::Id<'source>) -> Result<Variable, Error<'source>> {
        match self.variables.get(&id.id) {
            Some(variable) => Ok(variable.clone()),

            None => {
                let error = Error::non_existent_variable(id.clone());

                Err(error)
            }
        }
    }
}
