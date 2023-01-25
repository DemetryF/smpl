use itertools::Itertools;

use crate::{
    parser::ast::statement::function_statement::FunctionStatement,
    static_analyzer::{check::Check, env::Env, StaticAnalyzer},
};

impl Check for FunctionStatement {
    fn check(&self, analyzer: &mut StaticAnalyzer, _: &mut Env) {
        self.check_args(analyzer);

        analyzer.add_function(self.id.clone(), self.args.len());

        let mut env = self.env_from_args();

        self.body.check(analyzer, &mut env);
    }
}

impl FunctionStatement {
    fn check_args(&self, analyzer: &mut StaticAnalyzer) {
        let mut duplicates = self.args.iter().duplicates_by(|x| &x.value);

        match duplicates.next() {
            None => (),
            Some(id) => analyzer.duplicate_args_error(id.to_owned()),
        }
    }

    fn env_from_args(&self) -> Env {
        let mut env = Env::new();

        for arg in self.args.iter() {
            env.add_variable(arg.to_owned());
        }

        env
    }
}
