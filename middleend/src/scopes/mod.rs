mod function;
mod scope;
mod variable;

use std::collections::HashMap;

use crate::{error::Error, instruction::Id, scopes::scope::Scope};

pub use self::{function::Function, variable::Variable};

pub struct Scopes {
    envs: Vec<Scope>,
    stack: Vec<usize>,

    functions: HashMap<String, Function>,

    variables_counter: usize,
}

impl Scopes {
    fn current(&mut self) -> &mut Scope {
        let index = *self.stack.last().unwrap();

        &mut self.envs[index]
    }

    pub fn fork(&mut self) {
        let new_env_num = self.envs.len();

        self.stack.push(new_env_num);
        self.envs.push(Scope::default())
    }

    pub fn exit(&mut self) {
        self.stack.pop();
    }

    pub fn add_variable(&mut self, id: smplc_ast::Id) -> Result<Id, Error> {
        if let Ok(variable) = self.current().get(&id) {
            let error = Error::redeclaring_variable(id, variable);

            return Err(error);
        }

        let new_id = Id::new(format!("${}", self.inc_variables_counter()));

        let new_variable = Variable {
            defined_at: id.pos,
            id: new_id.clone(),
        };

        self.current().variables.insert(id.id, new_variable);

        Ok(new_id)
    }

    pub fn inc_variables_counter(&mut self) -> usize {
        let variables_count = self.variables_counter;

        self.variables_counter += 1;

        variables_count
    }

    pub fn get_variable(&self, id: smplc_ast::Id) -> Result<Variable, Error> {
        for i in self.stack.iter().rev().copied() {
            let env = &self.envs[i];

            if let Ok(variable) = env.get(&id) {
                return Ok(variable);
            }
        }

        Err(Error::non_existent_variable(id))
    }

    pub fn get_function(&self, id: &smplc_ast::Id) -> Result<&Function, Error> {
        match self.functions.get(&id.id) {
            Some(function) => Ok(function),
            None => {
                let error = Error::non_existent_function(id.clone());

                Err(error)
            }
        }
    }

    pub fn add_function(&mut self, id: &smplc_ast::Id, function: Function) -> Result<(), Error> {
        if self.get_function(id).is_ok() {
            let error = Error::non_existent_function(id.clone());

            return Err(error);
        }

        self.functions.insert(id.id.clone(), function);

        Ok(())
    }
}

impl Default for Scopes {
    fn default() -> Self {
        Self {
            envs: vec![Scope::default()],
            stack: vec![0],
            functions: HashMap::default(),
            variables_counter: 0,
        }
    }
}
