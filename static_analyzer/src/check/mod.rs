use super::{env::*, StaticAnalyzer};

pub mod block;
pub mod expr;
pub mod statement;

pub trait Check {
    fn check(&self, analyzer: &mut StaticAnalyzer, env: SharedEnv);
}
