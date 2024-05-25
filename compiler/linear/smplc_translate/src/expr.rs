use smplc_hir::{self as hir, NumberType};
use smplc_lir::{Assign, AssignRhs, Atom, Call, FunctionId, Goto, If, Label};

use crate::translator::Translator;

pub fn translate_expr(expr: hir::Expr, translator: &mut Translator) -> AssignRhs {
    match expr {
        hir::Expr::Binary { lhs, op, rhs } => {
            if let hir::BinOp::Arithm(op, ty) = op {
                let rhs = translate_expr(*rhs, translator);
                let rhs = atom_or_assign(translator, rhs);

                let lhs = translate_expr(*lhs, translator);
                let lhs = atom_or_assign(translator, lhs);

                AssignRhs::Binary { lhs, op, rhs, ty }
            } else {
                let true_label = translator.next_label();
                let false_label = translator.next_label();
                let end_label = translator.next_label();

                let expr = hir::Expr::Binary { lhs, op, rhs };

                translate_logic_expr(expr, translator, true_label.clone(), false_label.clone());

                let result = translator.variables.next_id(NumberType::Int);

                translator.code.add_label(false_label.clone());

                translator.code.push(Assign {
                    lhs: result,
                    rhs: AssignRhs::Atom(Atom::Int(0)),
                });

                translator.code.push(Goto {
                    label: end_label.clone(),
                });

                translator.code.add_label(true_label);

                translator.code.push(Assign {
                    lhs: result,
                    rhs: AssignRhs::Atom(Atom::Int(1)),
                });

                translator.code.add_label(end_label);

                AssignRhs::Atom(Atom::Id(result))
            }
        }

        hir::Expr::Unary { op, rhs } => match op {
            hir::UnOp::Not => todo!(),
            hir::UnOp::Neg(ty) => {
                let rhs = translate_expr(*rhs, translator);
                let rhs = atom_or_assign(translator, rhs);

                AssignRhs::Neg { rhs, ty }
            }
        },

        hir::Expr::Call { fun_ref, args } => {
            let args = translate_args(translator, args, &fun_ref.args);

            AssignRhs::Call(
                Call {
                    id: FunctionId(fun_ref.id.clone()),
                    args,
                },
                NumberType::for_ir(fun_ref.ret_ty.unwrap()),
            )
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
            if let hir::BinOp::Rel(op, ty) = op {
                let lhs = translate_expr(*lhs, translator);
                let lhs = atom_or_assign(translator, lhs);

                let rhs = translate_expr(*rhs, translator);
                let rhs = atom_or_assign(translator, rhs);

                translator.code.push(If {
                    lhs,
                    op,
                    ty,
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

pub fn translate_call(
    translator: &mut Translator,
    id: FunctionId,
    args: Vec<hir::Expr>,
    args_types: &Vec<hir::Type>,
) {
    let args = translate_args(translator, args, args_types);

    translator.code.push(Call { id, args })
}

pub fn translate_args(
    translator: &mut Translator,
    args: Vec<hir::Expr>,
    args_types: &Vec<hir::Type>,
) -> Vec<(Atom, NumberType)> {
    args.into_iter()
        .zip(args_types)
        .map(|(arg, &ty)| {
            let arg = translate_expr(arg, translator);
            let arg = atom_or_assign(translator, arg);

            (arg, NumberType::for_ir(ty))
        })
        .collect()
}

pub fn translate_atom(translator: &mut Translator, atom: hir::Atom) -> Atom {
    match atom {
        hir::Atom::Var(var_ref) => Atom::Id(translator.variables.get(var_ref)),
        hir::Atom::Literal(literal) => match literal {
            hir::Literal::Real(num) => Atom::Real(num),
            hir::Literal::Int(num) => Atom::Int(num),
            hir::Literal::Bool(bool) => Atom::Int(bool as i32),
        },
    }
}

pub fn atom_or_assign(translator: &mut Translator, rhs: AssignRhs) -> Atom {
    if let AssignRhs::Atom(atom) = rhs {
        atom
    } else {
        let result = translator.variables.next_id(rhs_ty(&rhs, &translator));

        translator.code.push(Assign { lhs: result, rhs });

        Atom::Id(result)
    }
}

pub fn rhs_ty(rhs: &AssignRhs, translator: &Translator) -> NumberType {
    match rhs {
        &AssignRhs::Binary { ty, .. } => ty,
        &AssignRhs::Neg { ty, .. } => ty,
        &AssignRhs::Call(_, ty) => ty,
        AssignRhs::Atom(atom) => match atom {
            Atom::Real(_) => NumberType::Real,
            Atom::Int(_) => NumberType::Int,
            &Atom::Id(id) => translator.variables.ty(id),
        },
    }
}
