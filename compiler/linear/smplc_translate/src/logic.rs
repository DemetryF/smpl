use smplc_lir::{Atom, ControlFlow, Label, RelOp, Type, Value};
use smplc_thir as thir;
use smplc_thir::Symbols;

use crate::{
    call::translate_call,
    expr::{translate_atom, translate_expr},
    idents::BaseIdents,
    translator::Translator,
};

pub fn translate_logic(
    expr: thir::Expr,
    translator: &mut Translator,
    idents: &mut BaseIdents,
    symbols: &Symbols,
    true_label: Label,
    false_label: Label,
) {
    match expr {
        thir::Expr::Binary {
            op: thir::BinOp::Rel(op, ty),
            lhs,
            rhs,
        } => {
            let lhs = translate_expr(*lhs, translator, idents, symbols);
            let rhs = translate_expr(*rhs, translator, idents, symbols);

            translator.code.push(ControlFlow::If {
                lhs: Atom::Id(lhs),
                op,
                ty: ty.into(),
                rhs: Atom::Id(rhs),
                label: true_label,
            });

            translator
                .code
                .push(ControlFlow::Goto { label: false_label })
        }

        thir::Expr::Binary {
            op: thir::BinOp::And,
            lhs,
            rhs,
        } => {
            let lhs_true = translator.next_label();

            translate_logic(*lhs, translator, idents, symbols, lhs_true, false_label);

            translator.code.label(lhs_true);

            translate_logic(*rhs, translator, idents, symbols, true_label, false_label);
        }

        thir::Expr::Binary {
            op: thir::BinOp::Or,
            lhs,
            rhs,
        } => {
            let lhs_false = translator.next_label();

            translate_logic(*lhs, translator, idents, symbols, true_label, lhs_false);

            translator.code.label(lhs_false);

            translate_logic(*rhs, translator, idents, symbols, true_label, false_label);
        }

        thir::Expr::Unary {
            op: thir::UnOp::Not,
            rhs,
        } => {
            translate_logic(*rhs, translator, idents, symbols, false_label, true_label);
        }

        thir::Expr::Call { fun, args } => {
            let result = idents.next(Type::Int);

            translate_call(translator, idents, symbols, Some(result), fun, args);

            translator.code.push(ControlFlow::If {
                lhs: Atom::Id(result),
                op: RelOp::Eq,
                ty: Type::Int,
                rhs: Atom::Number(Value::Int(1)),
                label: true_label,
            });

            translator
                .code
                .push(ControlFlow::Goto { label: false_label });
        }

        thir::Expr::Atom(atom) => {
            let value = translate_atom(atom, idents);

            translator.code.push(ControlFlow::If {
                lhs: value,
                op: RelOp::Eq,
                ty: Type::Int,
                rhs: Atom::Number(Value::Int(1)),
                label: true_label,
            });

            translator
                .code
                .push(ControlFlow::Goto { label: false_label });
        }

        _ => unreachable!(),
    }
}
