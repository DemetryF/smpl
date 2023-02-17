use std::{cell::RefCell, rc::Rc};

use frontend::ast::Block;

use crate::{Check, Env, SharedEnv, StaticAnalyzer};

impl Check for Block {
    fn check(&self, analyzer: &mut StaticAnalyzer, env: SharedEnv) {
        let next_env = SharedEnv::new(RefCell::new(Env::new_with_parent(env)));

        for stmt in self.0.iter() {
            stmt.check(analyzer, Rc::clone(&next_env));
        }
    }
}
