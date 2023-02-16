use crate::{
    ast::Expr,
    static_analyzer::{env::*, StaticAnalyzer},
};

use super::Check;

pub mod atom;
pub mod call;
pub mod infix;
pub mod prefix;

impl Check for Expr {
    fn check(&self, analyzer: &mut StaticAnalyzer, env: SharedEnv) {
        match self {
            Expr::Atom(atom) => atom.check(analyzer, env),
            Expr::Infix(infix) => infix.check(analyzer, env),
            Expr::Call(call) => call.check(analyzer, env),
            Expr::Prefix(prefix) => prefix.check(analyzer, env),
        }
    }
}
