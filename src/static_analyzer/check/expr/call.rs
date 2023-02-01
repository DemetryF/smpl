use crate::{
    ast::Call,
    static_analyzer::{check::Check, env::Env, StaticAnalyzer},
};

impl Check for Call {
    fn check(&self, analyzer: &mut StaticAnalyzer, env: &mut Env) {
        match analyzer.functions.get_mut(&self.id.value) {
            Some(_) => self.check_args_count(analyzer),
            None => analyzer.non_existing_function_error(self.id.to_owned()),
        }

        self.check_args(analyzer, env);
    }
}

impl Call {
    fn check_args(&self, analyzer: &mut StaticAnalyzer, env: &mut Env) {
        for arg in self.args.iter() {
            arg.check(analyzer, env);
        }
    }

    fn check_args_count(&self, analyzer: &mut StaticAnalyzer) {
        let func = analyzer.functions.get_mut(&self.id.value).unwrap();

        if func.args_count != self.args.len() {
            func.uses_count += 1;
            analyzer.invalid_args_count_error(self);
        }
    }
}
