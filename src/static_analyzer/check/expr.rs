use crate::{
    parser::ast::expr::{call::Call, unary::Unary, Atom, Binary, Expr},
    static_analyzer::{env::Env, static_error::StaticError, StaticAnalyzer},
};

use super::Check;

impl Check for Expr {
    fn check(&self, analyzer: &mut StaticAnalyzer, env: &mut Env) {
        match self {
            Expr::Atom(atom) => atom.check(analyzer, env),
            Expr::Binary(binary) => binary.check(analyzer, env),
            Expr::Call(call) => call.check(analyzer, env),
            Expr::Unary(unary) => unary.check(analyzer, env),
        }
    }
}

impl Check for Atom {
    fn check(&self, analyzer: &mut StaticAnalyzer, env: &mut Env) {
        if let Atom::Id(id) = &self {
            if !env.search(id) {
                analyzer.errors.push(StaticError::NonExistingVariable);
            }
        }
    }
}

impl Check for Binary {
    fn check(&self, analyzer: &mut StaticAnalyzer, env: &mut Env) {
        self.lhs.check(analyzer, env);
        self.rhs.check(analyzer, env);
    }
}

impl Check for Call {
    fn check(&self, analyzer: &mut StaticAnalyzer, env: &mut Env) {
        match analyzer.functions.get(&self.id.value) {
            Some(func) => {
                if func.args_count != self.args.len() {
                    analyzer.errors.push(StaticError::InvalidArgumentsCount)
                }
            }
            None => analyzer.errors.push(StaticError::NonExistingFunction),
        }

        for arg in self.args.iter() {
            arg.check(analyzer, env);
        }
    }
}

impl Check for Unary {
    fn check(&self, analyzer: &mut StaticAnalyzer, env: &mut Env) {
        self.rhs.check(analyzer, env);
    }
}
