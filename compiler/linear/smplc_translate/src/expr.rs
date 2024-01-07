use smplc_hir as hir;
use smplc_hir::Expr;
use smplc_ir::{Atom, Binary, Call, Copy, FunctionId, Id, Param, Unary};

use crate::translator::Translator;

pub fn translate_expr_and_write_in(expr: Expr, translator: &mut Translator, result: Id) {
    match expr {
        Expr::Binary { lhs, op, rhs } => {
            let lhs = translate_expr(*lhs, translator);
            let rhs = translate_expr(*rhs, translator);

            translator.code.push(Binary {
                result,
                lhs,
                op,
                rhs,
            });
        }

        Expr::Unary { op, rhs } => {
            let rhs = translate_expr(*rhs, translator);

            translator.code.push(Unary { result, op, rhs });
        }

        Expr::Call { fun_ref, args } => {
            translate_call(
                translator,
                FunctionId(fun_ref.id.clone()),
                args,
                Some(result),
            );
        }

        Expr::Atom(atom) => {
            let value = translate_atom(translator, atom);

            translator.code.push(Copy { result, value });
        }
    }
}

pub fn translate_expr(expr: Expr, translator: &mut Translator) -> Atom {
    match expr {
        Expr::Binary { lhs, op, rhs } => {
            let lhs = translate_expr(*lhs, translator);
            let rhs = translate_expr(*rhs, translator);

            let result = translator.variables.next_id();

            translator.code.push(Binary {
                result,
                lhs,
                op,
                rhs,
            });

            Atom::Id(result)
        }

        Expr::Unary { op, rhs } => {
            let rhs = translate_expr(*rhs, translator);

            let result = translator.variables.next_id();

            translator.code.push(Unary { result, op, rhs });

            Atom::Id(result)
        }

        Expr::Call { fun_ref, args } => {
            let result = translator.variables.next_id();

            translate_call(
                translator,
                FunctionId(fun_ref.id.clone()),
                args,
                Some(result),
            );

            Atom::Id(result)
        }

        Expr::Atom(atom) => translate_atom(translator, atom),
    }
}

pub fn translate_call(
    translator: &mut Translator,
    id: FunctionId,
    args: Vec<Expr>,
    result: Option<Id>,
) {
    args.into_iter()
        .map(|arg| translate_expr(arg, translator))
        .rev()
        .collect::<Vec<_>>()
        .into_iter()
        .for_each(|value| translator.code.push(Param { value }));

    translator.code.push(Call { result, id });
}

pub fn translate_atom(translator: &mut Translator, atom: hir::Atom) -> Atom {
    match atom {
        hir::Atom::Var(var_ref) => Atom::Id(translator.variables.get(var_ref)),
        hir::Atom::Literal(literal) => Atom::Number(literal.into()),
    }
}
