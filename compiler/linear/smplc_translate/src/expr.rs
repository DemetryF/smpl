use smplc_hir as hir;
use smplc_lir::{Assign, AssignRhs, Atom, Call, FunctionId};

use crate::translator::Translator;

pub fn translate_expr(expr: hir::Expr, translator: &mut Translator) -> AssignRhs {
    match expr {
        hir::Expr::Binary { lhs, op, rhs } => {
            let lhs = translate_expr(*lhs, translator);
            let lhs = atom_or_assign(translator, lhs);

            let rhs = translate_expr(*rhs, translator);
            let rhs = atom_or_assign(translator, rhs);

            AssignRhs::Binary { lhs, op, rhs }
        }

        hir::Expr::Unary { op, rhs } => {
            let rhs = translate_expr(*rhs, translator);
            let rhs = atom_or_assign(translator, rhs);

            AssignRhs::Unary { op, rhs }
        }

        hir::Expr::Call { fun_ref, args } => {
            let args = translate_args(translator, args);

            AssignRhs::Call(Call {
                id: FunctionId(fun_ref.id.clone()),
                args,
            })
        }

        hir::Expr::Atom(atom) => AssignRhs::Atom(translate_atom(translator, atom)),
    }
}

pub fn translate_call(translator: &mut Translator, id: FunctionId, args: Vec<hir::Expr>) {
    let args = translate_args(translator, args);

    translator.code.push(Call { id, args })
}

pub fn translate_args(translator: &mut Translator, args: Vec<hir::Expr>) -> Vec<Atom> {
    args.into_iter()
        .map(|arg| {
            let arg = translate_expr(arg, translator);
            atom_or_assign(translator, arg)
        })
        .collect()
}

pub fn translate_atom(translator: &mut Translator, atom: hir::Atom) -> Atom {
    match atom {
        hir::Atom::Var(var_ref) => Atom::Id(translator.variables.get(var_ref)),
        hir::Atom::Literal(literal) => Atom::Number(literal.into()),
    }
}

pub fn atom_or_assign(translator: &mut Translator, rhs: AssignRhs) -> Atom {
    if let AssignRhs::Atom(atom) = rhs {
        atom
    } else {
        let result = translator.variables.next_id();

        translator.code.push(Assign { lhs: result, rhs });

        Atom::Id(result)
    }
}
