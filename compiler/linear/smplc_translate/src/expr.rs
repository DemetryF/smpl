use smplc_lir::{Assign, AssignRhs, Atom, Call, FunctionId, Goto, If, Label, RelOp};
use smplc_thir::NumberType;
use smplc_thir::{self as thir, Symbols};

use crate::translator::Translator;

pub fn translate_expr(
    expr: thir::Expr,
    translator: &mut Translator,
    symbols: &Symbols,
) -> AssignRhs {
    match expr {
        thir::Expr::Binary { lhs, op, rhs } => {
            if let thir::BinOp::Arithm(op, ty) = op {
                let rhs = translate_expr(*rhs, translator, symbols);
                let rhs = atom_or_assign(translator, rhs);

                let lhs = translate_expr(*lhs, translator, symbols);
                let lhs = atom_or_assign(translator, lhs);

                AssignRhs::Binary { lhs, op, rhs, ty }
            } else {
                let true_label = translator.next_label();
                let false_label = translator.next_label();
                let end_label = translator.next_label();

                let expr = thir::Expr::Binary { lhs, op, rhs };

                translate_logic_expr(
                    expr,
                    translator,
                    symbols,
                    true_label.clone(),
                    false_label.clone(),
                );

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

        thir::Expr::Unary { op, rhs } => match op {
            thir::UnOp::Not => todo!(),
            thir::UnOp::Neg(ty) => {
                let rhs = translate_expr(*rhs, translator, symbols);
                let rhs = atom_or_assign(translator, rhs);

                AssignRhs::Neg { rhs, ty }
            }
        },

        thir::Expr::Call { fun: fun_id, args } => {
            let fun = &symbols.functions[fun_id];

            let args = translate_args(translator, symbols, args, &fun.args_types);

            AssignRhs::Call(
                Call {
                    id: FunctionId(fun.id.0.into()),
                    args,
                },
                NumberType::for_ir(fun.ret_ty.unwrap()),
            )
        }

        thir::Expr::Atom(atom) => AssignRhs::Atom(translate_atom(translator, atom)),
    }
}

pub fn translate_logic_expr(
    expr: thir::Expr,
    translator: &mut Translator,
    symbols: &Symbols,
    true_label: Label,
    false_label: Label,
) {
    match expr {
        thir::Expr::Binary { lhs, op, rhs } => {
            if let thir::BinOp::Rel(op, ty) = op {
                let lhs = translate_expr(*lhs, translator, symbols);
                let lhs = atom_or_assign(translator, lhs);

                let rhs = translate_expr(*rhs, translator, symbols);
                let rhs = atom_or_assign(translator, rhs);

                translator.code.push(If {
                    lhs,
                    op,
                    ty,
                    rhs,
                    then_label: Some(true_label),
                    else_label: Some(false_label),
                })
            } else if op == thir::BinOp::And {
                let lhs_false_label = translator.next_label();

                translate_logic_expr(
                    *lhs,
                    translator,
                    symbols,
                    true_label.clone(),
                    lhs_false_label.clone(),
                );

                translator.code.add_label(lhs_false_label);

                translate_logic_expr(*rhs, translator, symbols, true_label, false_label);
            } else if op == thir::BinOp::Or {
                let lhs_true_label = translator.next_label();

                translate_logic_expr(
                    *lhs,
                    translator,
                    symbols,
                    lhs_true_label.clone(),
                    false_label.clone(),
                );

                translator.code.add_label(lhs_true_label);

                translate_logic_expr(*rhs, translator, symbols, true_label, false_label);
            } else {
                panic!("non bool expression in translate_logic_expr fn");
            }
        }

        thir::Expr::Unary { op, rhs } => {
            if op == thir::UnOp::Not {
                translate_logic_expr(*rhs, translator, symbols, false_label, true_label);
            } else {
                panic!("non bool expression in translate_logic_expr fn");
            }
        }

        thir::Expr::Atom(thir::Atom::Literal(lit)) => match lit.value {
            "true" => translator.code.push(Goto { label: true_label }),
            "false" => translator.code.push(Goto { label: true_label }),

            _ => unreachable!(),
        },

        thir::Expr::Atom(thir::Atom::Var(var)) => {
            translator.code.push(If {
                lhs: Atom::Id(translator.variables.get(var)),
                op: RelOp::Eq,
                ty: NumberType::Int,
                rhs: Atom::Int(1),
                then_label: Some(true_label),
                else_label: Some(false_label),
            });
        }

        _ => unreachable!(),
    }
}

pub fn translate_call(
    translator: &mut Translator,
    symbols: &Symbols,
    id: FunctionId,
    args: Vec<thir::Expr>,
    args_types: &Vec<thir::Type>,
) {
    let args = translate_args(translator, symbols, args, args_types);

    translator.code.push(Call { id, args })
}

pub fn translate_args(
    translator: &mut Translator,
    symbols: &Symbols,
    args: Vec<thir::Expr>,
    args_types: &Vec<thir::Type>,
) -> Vec<(Atom, NumberType)> {
    args.into_iter()
        .zip(args_types)
        .map(|(arg, &ty)| {
            let arg = translate_expr(arg, translator, symbols);
            let arg = atom_or_assign(translator, arg);

            (arg, NumberType::for_ir(ty))
        })
        .collect()
}

pub fn translate_atom(translator: &mut Translator, atom: thir::Atom) -> Atom {
    match atom {
        thir::Atom::Var(var_ref) => Atom::Id(translator.variables.get(var_ref)),
        thir::Atom::Literal(literal) => match literal.ty {
            thir::Type::Real => Atom::Real(parse_int::parse(literal.value).unwrap()),
            thir::Type::Int => Atom::Int(parse_int::parse(literal.value).unwrap()),
            thir::Type::Bool => Atom::Int(if literal.value == "true" { 1 } else { 0 }),
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
