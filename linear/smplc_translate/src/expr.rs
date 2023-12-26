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

        Expr::Call { function, args } => {
            translate_call(translator, function.id.clone(), args, Some(result));
        }

        Expr::Atom(atom) => {
            let value = match atom {
                smplc_hir::Atom::Var(id) => Atom::Id(id.id),
                smplc_hir::Atom::Value(value) => Atom::Number(value),
            };

            translator.code.push(Copy { result, value });
        }
    }
}

pub fn translate_expr(expr: Expr, translator: &mut Translator) -> Atom {
    match expr {
        Expr::Binary { lhs, op, rhs } => {
            let lhs = translate_expr(*lhs, translator);
            let rhs = translate_expr(*rhs, translator);

            let result = translator.next_id();

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

            let result = translator.next_id();

            translator.code.push(Unary { result, op, rhs });

            Atom::Id(result)
        }

        Expr::Call { function, args } => {
            let result = translator.next_id();

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
        let value = translate_expr(arg, translator);
        translator.code.push(Param { value })
    }

    translator.code.push(Call { result, id });
}
