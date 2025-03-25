pub mod error;

mod env;
mod inited;
mod semcheck;

#[cfg(test)]
mod tests;

use smplc_ast as ast;
use smplc_ast::Span;
use smplc_hir::{Symbols, Type, HIR};

use env::Env;
use error::SemResult;
use inited::GeneralInited;
use semcheck::{RawType, SemCheck};

pub fn sem_check(ast: Vec<ast::Declaration>) -> SemResult<HIR> {
    let mut env = Env::default();
    env.variables.fork();

    init_std(&mut env);

    let mut hir = HIR::default();

    for declaration in ast.iter() {
        if let ast::Declaration::Function(function) = declaration {
            let args_types = function
                .args
                .iter()
                .map(|arg| RawType(arg.ty).checked())
                .collect::<Result<_, _>>()?;

            let ret_ty = function
                .ret_ty
                .map(|arg| RawType(arg).checked())
                .transpose()?;

            env.functions.add(function.id, args_types, ret_ty)?;
        }
    }

    let mut inited = GeneralInited::default();

    for declaration in ast {
        match declaration {
            ast::Declaration::Function(function) => {
                env.current_fn = Some(env.functions.get(function.id).unwrap());

                hir.functions.push(function.check(&mut env, &mut inited)?);
            }

            ast::Declaration::Constant(constant) => {
                hir.constants.push(constant.check(&mut env, &mut inited)?);
            }
        }
    }

    hir.symbols = Symbols {
        functions: env.functions.symbols,
        variables: env.variables.symbols,
    };

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
