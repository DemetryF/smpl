use crate::{
    ast::WhileStatement,
    static_analyzer::{check::Check, env::Env, StaticAnalyzer},
};

impl Check for WhileStatement {
    fn check(&self, analyzer: &mut StaticAnalyzer, env: &mut Env) {
        self.cond.check(analyzer, env);
        self.body.check(analyzer, env);
    }
}
