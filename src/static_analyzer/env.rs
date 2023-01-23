use std::collections::HashMap;

use crate::lexer::pos::Pos;

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

    pub fn search(&self, id: &String) -> bool {
        if self.variables.contains_key(id) {
            return true;
        }

        match &self.parent {
            Some(env) => env.search(id),
            None => false,
        }
    }
}
