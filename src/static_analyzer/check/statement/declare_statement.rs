use crate::{
    parser::ast::statement::declare_statement::DeclareStatement,
    static_analyzer::{
        check::Check,
        env::{Env, StaticIdInfo},
        static_error::{StaticError, StaticErrorKind},
        StaticAnalyzer,
    },
};

impl Check for DeclareStatement {
    fn check(&self, analyzer: &mut StaticAnalyzer, env: &mut Env) {
        match env.variables.get(&self.id.value) {
            None => {
                env.variables.insert(
                    self.id.value.clone(),
                    StaticIdInfo {
                        define_pos: self.id.pos,
                        uses_count: 0,
                    },
                );
            }
            Some(id) => {
                analyzer.errors.push(StaticError::new(
                    StaticErrorKind::ReDeclaringVariable {
                        name: self.id.value.clone(),
                        defined_at: id.define_pos,
                    },
                    self.id.pos,
                ));
            }
        }

        if let Some(expr) = &self.expr {
            expr.check(analyzer, env);
        }
    }
}
