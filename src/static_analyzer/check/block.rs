use std::{cell::RefCell, rc::Rc};

use crate::{
    ast::Block,
    static_analyzer::{
        env::{Env, SharedEnv},
        StaticAnalyzer,
    },
};

use super::Check;

impl Check for Block {
    fn check(&self, analyzer: &mut StaticAnalyzer, env: SharedEnv) {
        let next_env = SharedEnv::new(RefCell::new(Env::new_with_parent(env)));

        for stmt in self.0.iter() {
            stmt.check(analyzer, Rc::clone(&next_env));
        }
    }
}
