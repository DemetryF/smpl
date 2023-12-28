pub mod error;

mod env;
mod semcheck;
#[cfg(test)]
mod tests;

use smplc_ast as ast;
use smplc_hir::{FunData, FunRef, Function, HIR};

use env::Env;
use error::SemResult;
use semcheck::SemCheck;

pub fn sem_check<'source>(ast: Vec<ast::Statement<'source>>) -> SemResult<'source, HIR> {
    let (functions, stmts) = separate_functions(ast);

    let mut env = Env::default();

    env.functions
        .add(smplc_ast::Id::new("print", smplc_ast::Pos::default()), 1)
        .unwrap();

    let mut hir = HIR::default();

    for function in functions.iter() {
        env.functions.add(function.id, function.args.len())?;
    }

    for ast::FunctionStatement { id, args, body } in functions {
        let fun_ref = env.functions.get(id).unwrap();

        hir.functions
            .push(proccess_function(&mut env, fun_ref, args, body)?);
    }

    let fun_ref = FunRef::new(FunData {
        declared_at: ast::Pos::default(),
        id: "main".into(),
        args_count: 0,
    });

    hir.functions.push(proccess_function(
        &mut env,
        fun_ref,
        vec![],
        ast::Block { stmts },
    )?);

    hir.variables_count = env.variables.count();

    Ok(hir)
}

pub fn separate_functions(
    ast: Vec<ast::Statement>,
) -> (Vec<ast::FunctionStatement>, Vec<ast::Statement>) {
    let mut functions = Vec::new();
    let mut others = Vec::new();

    for statement in ast {
        if let ast::Statement::Function(function) = statement {
            functions.push(function)
        } else {
            others.push(statement)
        }
    }

    (functions, others)
}

pub fn proccess_function<'source>(
    env: &mut Env<'source>,
    fun_ref: FunRef,
    args: Vec<ast::Id<'source>>,
    body: ast::Block<'source>,
) -> SemResult<'source, Function> {
    env.variables.fork();

    let args = args
        .into_iter()
        .map(|arg| env.variables.add_argument(arg))
        .collect::<Result<Vec<_>, _>>()?;

    let statements = body.check(env)?.statements;

    env.variables.exit();

    Ok(Function {
        function: fun_ref,
        args,
        statements,
    })
}
