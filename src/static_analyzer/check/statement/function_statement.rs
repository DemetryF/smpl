use itertools::Itertools;

use crate::{
    parser::ast::statement::function_statement::FunctionStatement,
    static_analyzer::{
        check::Check,
        env::{Env, StaticIdInfo},
        static_error::StaticError,
        StaticAnalyzer, StaticFunctionInfo,
    },
};

impl Check for FunctionStatement {
    fn check(&self, analyzer: &mut StaticAnalyzer, _: &mut Env) {
        let mut f_env = Env::new();
        self.check_args(analyzer);

        analyzer.functions.insert(
            self.id.value.clone(),
            StaticFunctionInfo {
                uses_count: 0,
                args_count: self.args.len(),
            },
        );

        for arg in self.args.iter() {
            f_env.variables.insert(
                arg.value.clone(),
                StaticIdInfo {
                    define_pos: arg.pos,
                    uses_count: 0,
                },
            );
        }

        self.body.check(analyzer, &mut f_env);
    }
}

impl FunctionStatement {
    fn check_args(&self, analyzer: &mut StaticAnalyzer) {
        let mut duplicates = self.args.iter().duplicates_by(|x| &x.value);

        if duplicates.next().is_some() {
            analyzer.errors.push(StaticError::DuplicatesFunctionArgs)
        }
    }
}
