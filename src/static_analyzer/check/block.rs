use crate::{
    ast::Block,
    static_analyzer::{env::Env, StaticAnalyzer},
};

use super::Check;

impl Check for Block {
    fn check(&self, analyzer: &mut StaticAnalyzer, env: &mut Env) {
        let mut next_env = Env::new_with_parent(Box::new(env.to_owned()));

        for stmt in self.0.iter() {
            stmt.check(analyzer, &mut next_env);
        }
    }
}
