use smplc_hir::Expr;
use smplc_ir::{Atom, Binary, Call, FunctionId, Id, Param, Unary};

use crate::translator::Translator;

pub fn translate_expr(expr: Expr, translator: &mut Translator, result: Id) -> Atom {
    match expr {
        Expr::Binary { lhs, op, rhs } => {
            let lhs_result = translator.next_id();
            let lhs = translate_expr(*lhs, translator, lhs_result);

            let rhs_result = translator.next_id();
            let rhs = translate_expr(*rhs, translator, rhs_result);

            translator.code.push(Binary {
                result,
                lhs,
                op,
                rhs,
            });

            Atom::Id(result)
        }

        Expr::Unary { op, rhs } => {
            let rhs_result = translator.next_id();
            let rhs = translate_expr(*rhs, translator, rhs_result);

            translator.code.push(Unary { result, op, rhs });

            Atom::Id(result)
        }

        Expr::Call { function, args } => {
            translate_call(translator, function.id.clone(), args, Some(result));

            Atom::Id(result)
        }

        Expr::Atom(atom) => match atom {
            smplc_hir::Atom::Var(id) => Atom::Id(id.id),
            smplc_hir::Atom::Value(value) => Atom::Number(value),
        },
    }
}

pub fn translate_call(
    translator: &mut Translator,
    id: FunctionId,
    args: Vec<Expr>,
    result: Option<Id>,
) {
    for arg in args {
        let result = translator.next_id();

        translate_expr(arg, translator, result);

        translator.code.push(Param {
            value: Atom::Id(result),
        })
    }

    translator.code.push(Call { result, id });
}
