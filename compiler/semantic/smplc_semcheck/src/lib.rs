pub mod error;

mod env;
mod semcheck;
#[cfg(test)]
mod tests;

use smplc_ast as ast;
use smplc_hir::HIR;

use env::Env;
use error::SemResult;
use semcheck::SemCheck;

pub fn sem_check(ast: Vec<ast::Declaration>) -> SemResult<HIR> {
    let mut env = Env::default();
    env.variables.fork();

    env.functions
        .add(ast::Id::new("print", ast::Pos::default()), 1)
        .unwrap();

    let mut hir = HIR::default();

    for declaration in ast.iter() {
        if let ast::Declaration::Function(function) = declaration {
            env.functions.add(function.id, function.args.len())?;
        }
    }

    for declaration in ast {
        match declaration {
            ast::Declaration::Function(function) => {
                hir.functions.push(function.check(&mut env)?);
            }

            ast::Declaration::Constant(constant) => {
                hir.constants.push(constant.check(&mut env)?);
            }
        }
    }

    Ok(hir)
}
