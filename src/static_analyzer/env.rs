use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{ast::Id, lexer::Pos};

#[derive(Debug, Clone)]
pub struct StaticIdInfo {
    pub define_pos: Pos,
    pub uses_count: usize,
}

#[derive(Debug, Clone)]
pub struct Env {
    pub parent: Option<SharedEnv>,
    pub variables: HashMap<String, StaticIdInfo>,
}

pub type SharedEnv = Rc<RefCell<Env>>;

impl Env {
    pub fn new() -> Self {
        Self {
            parent: None,
            variables: HashMap::new(),
        }
    }

    pub fn new_with_parent(parent: SharedEnv) -> Self {
        Self {
            parent: Some(parent),
            variables: HashMap::new(),
        }
    }

    pub fn search(&mut self, id: &String) -> bool {
        if self.variables.contains_key(id) {
            let var = self.variables.get_mut(id).unwrap();
            var.uses_count += 1;

            return true;
        }

        match self.parent.as_deref() {
            Some(env) => env.borrow_mut().search(id),
            None => false,
        }
    }

    pub fn add_variable(&mut self, id: Id) {
        self.variables.insert(
            id.value.clone(),
            StaticIdInfo {
                define_pos: id.pos,
                uses_count: 0,
            },
        );
    }
}
