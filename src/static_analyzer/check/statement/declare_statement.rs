use crate::{
    ast::DeclareStatement,
    static_analyzer::{check::Check, env::Env, StaticAnalyzer},
};

impl Check for DeclareStatement {
    fn check(&self, analyzer: &mut StaticAnalyzer, env: &mut Env) {
        match env.variables.get(&self.id.value) {
            None => env.add_variable(self.id.clone()),
            Some(id) => analyzer.redeclaring_error(self.id.clone(), id),
        }

        if let Some(expr) = &self.expr {
            expr.check(analyzer, env);
        }
    }
}
