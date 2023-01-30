use crate::{
    parser::ast::IfStatement,
    static_analyzer::{check::Check, env::Env, StaticAnalyzer},
};

impl Check for IfStatement {
    fn check(&self, analyzer: &mut StaticAnalyzer, env: &mut Env) {
        self.cond.check(analyzer, env);
        self.then_body.check(analyzer, env);

        if let Some(else_body) = &self.else_body {
            else_body.check(analyzer, env);
        }
    }
}
