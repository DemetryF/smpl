use std::collections::HashMap;

use crate::error::Error;

use super::Variable;

#[derive(Default)]
pub struct Scope {
    pub variables: HashMap<String, Variable>,
}

impl Scope {
    pub fn get(&self, id: &smplc_ast::Id) -> Result<Variable, Error> {
        match self.variables.get(&id.id) {
            Some(variable) => Ok(variable.clone()),

            None => {
                let error = Error::non_existent_variable(id.clone());

                Err(error)
            }
        }
    }
}
