use std::collections::HashMap;

use crate::{lexer::pos::Pos, parser::ast::Id};

#[derive(Debug, Clone)]
pub struct StaticIdInfo {
    pub define_pos: Pos,
    pub uses_count: usize,
}

#[derive(Debug, Clone)]
pub struct Env {
    pub parent: Option<Box<Env>>,
    pub variables: HashMap<String, StaticIdInfo>,
}

impl Env {
    pub fn new() -> Self {
        Self {
            parent: None,
            variables: HashMap::new(),
        }
    }

    pub fn new_with_parent(parent: Box<Env>) -> Self {
        Self {
            parent: Some(parent),
            variables: HashMap::new(),
        }
    }

    pub fn search(&mut self, id: &String) -> bool {
        if self.variables.contains_key(id) {
            let var = self.variables.get_mut(id).expect("");
            var.uses_count += 1;

            return true;
        }

        match &mut self.parent {
            Some(env) => env.search(id),
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
