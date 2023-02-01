use crate::{
    ast::Expr,
    static_analyzer::{env::Env, StaticAnalyzer},
};

use super::Check;

pub mod atom;
pub mod binary;
pub mod call;
pub mod unary;

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
