pub mod error;

mod env;
mod semcheck;

#[cfg(test)]
mod tests;

use smplc_ast as ast;
use smplc_ast::Span;
use smplc_hir::{Type, HIR};

use env::Env;
use error::SemResult;
use semcheck::SemCheck;

pub fn sem_check(ast: Vec<ast::Declaration>) -> SemResult<HIR> {
    let mut env = Env::default();
    env.variables.fork();

    init_std(&mut env);

    let mut hir = HIR::default();

    for declaration in ast.iter() {
        if let ast::Declaration::Function(function) = declaration {
            env.functions.add(
                function.id,
                function.args.iter().map(|arg| arg.ty).collect(),
                function.ret_ty,
            )?;
        }
    }

    for declaration in ast {
        match declaration {
            ast::Declaration::Function(function) => {
                env.current_fn = Some(env.functions.get(function.id).unwrap());

                hir.functions.push(function.check(&mut env)?);
            }

            ast::Declaration::Constant(constant) => {
                hir.constants.push(constant.check(&mut env)?);
            }
        }
    }

    Ok(hir)
}

pub fn init_std(env: &mut Env) {
    env.functions
        .add(
            ast::Id::new("printr", Span::default()),
            vec![Type::Real],
            None,
        )
        .unwrap();

    env.functions
        .add(
            ast::Id::new("printi", Span::default()),
            vec![Type::Int],
            None,
        )
        .unwrap();

    env.functions
        .add(
            ast::Id::new("printb", Span::default()),
            vec![Type::Bool],
            None,
        )
        .unwrap();
}
