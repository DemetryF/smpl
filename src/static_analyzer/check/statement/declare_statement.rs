use crate::{
    parser::ast::statement::declare_statement::DeclareStatement,
    static_analyzer::{
        check::Check,
        env::{Env, StaticIdInfo},
        static_error::StaticError,
        StaticAnalyzer,
    },
};

impl Check for DeclareStatement {
    fn check(&self, analyzer: &mut StaticAnalyzer, env: &mut Env) {
        if env.variables.contains_key(&self.id.value) {
            analyzer.errors.push(StaticError::ReDeclaringVariable);
        } else {
            env.variables.insert(
                self.id.value.clone(),
                StaticIdInfo {
                    define_pos: self.id.pos,
                    uses_count: 0,
                },
            );
        }

        if let Some(expr) = &self.expr {
            expr.check(analyzer, env);
        }
    }
}
