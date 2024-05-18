use smplc_hir as hir;
use smplc_lir::{Assign, AssignRhs, Atom, BinOp, Call, FunctionId, Goto, If, Label, RelOp};

use crate::translator::Translator;

pub fn translate_expr(expr: hir::Expr, translator: &mut Translator) -> AssignRhs {
    match expr {
        hir::Expr::Binary { lhs, op, rhs } => {
            if let Ok(op) = BinOp::try_from(op) {
                let rhs = translate_expr(*rhs, translator);
                let rhs = atom_or_assign(translator, rhs);

                let lhs = translate_expr(*lhs, translator);
                let lhs = atom_or_assign(translator, lhs);

                AssignRhs::Binary { lhs, op, rhs }
            } else {
                let true_label = translator.next_label();
                let false_label = translator.next_label();
                let end_label = translator.next_label();

                let expr = hir::Expr::Binary { lhs, op, rhs };

                translate_logic_expr(expr, translator, true_label.clone(), false_label.clone());

                let result = translator.variables.next_id();

                translator.code.add_label(false_label.clone());

                translator.code.push(Assign {
                    lhs: result,
                    rhs: AssignRhs::Atom(Atom::Number(0.0)),
                });

                translator.code.push(Goto {
                    label: end_label.clone(),
                });

                translator.code.add_label(true_label);

                translator.code.push(Assign {
                    lhs: result,
                    rhs: AssignRhs::Atom(Atom::Number(1.0)),
                });

                translator.code.add_label(end_label);

                AssignRhs::Atom(Atom::Id(result))
            }
        }

        hir::Expr::Unary { op, rhs } => match op {
            hir::UnOp::Not => todo!(),
            hir::UnOp::Neg => {
                let rhs = translate_expr(*rhs, translator);
                let rhs = atom_or_assign(translator, rhs);

                AssignRhs::Neg { rhs }
            }
        },

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

pub fn translate_logic_expr(
    expr: hir::Expr,
    translator: &mut Translator,
    true_label: Label,
    false_label: Label,
) {
    match expr {
        hir::Expr::Binary { lhs, op, rhs } => {
            if let Ok(op) = RelOp::try_from(op) {
                let lhs = translate_expr(*lhs, translator);
                let lhs = atom_or_assign(translator, lhs);

                let rhs = translate_expr(*rhs, translator);
                let rhs = atom_or_assign(translator, rhs);

                translator.code.push(If {
                    lhs,
                    op,
                    rhs,
                    then_label: Some(true_label),
                    else_label: Some(false_label),
                })
            } else if op == hir::BinOp::And {
                let lhs_false_label = translator.next_label();

                translate_logic_expr(
                    *lhs,
                    translator,
                    true_label.clone(),
                    lhs_false_label.clone(),
                );

                translator.code.add_label(lhs_false_label);

                translate_logic_expr(*rhs, translator, true_label, false_label);
            } else if op == hir::BinOp::Or {
                let lhs_true_label = translator.next_label();

                translate_logic_expr(
                    *lhs,
                    translator,
                    lhs_true_label.clone(),
                    false_label.clone(),
                );

                translator.code.add_label(lhs_true_label);

                translate_logic_expr(*rhs, translator, true_label, false_label);
            } else {
                panic!("non bool expression in translate_logic_expr fn");
            }
        }

        hir::Expr::Unary { op, rhs } => {
            if op == hir::UnOp::Not {
                translate_logic_expr(*rhs, translator, false_label, true_label);
            } else {
                panic!("non bool expression in translate_logic_expr fn");
            }
        }

        _ => unreachable!(),
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
