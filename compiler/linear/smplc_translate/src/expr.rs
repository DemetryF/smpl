use nalgebra::Vector2;

use smplc_lir::{Atom, ControlFlow, Id, Sequental, Type, UnOp, Value};
use smplc_thir::Symbols;
use smplc_thir::{self as thir, VecOp};

use crate::{
    call::translate_call, idents::BaseIdents, logic::translate_logic, translator::Translator,
};

pub fn translate_expr<'source>(
    expr: thir::Expr,
    translator: &mut Translator<'source>,
    idents: &mut BaseIdents,
    symbols: &Symbols<'source>,
) -> Id {
    match expr {
        thir::Expr::Binary {
            op: thir::BinOp::And | thir::BinOp::Or | thir::BinOp::Eq(..) | thir::BinOp::Ord(..),
            ..
        }
        | thir::Expr::Unary {
            op: thir::UnOp::Not,
            ..
        } => {
            let true_label = translator.next_label();
            let false_label = translator.next_label();
            let end_label = translator.next_label();

            translate_logic(expr, translator, idents, symbols, true_label, false_label);

            let result = idents.next(Type::Int);

            translator.code.label(true_label);
            translator.code.push(Sequental::Assign {
                dst: result,
                value: Atom::Value(Value::Int(1)),
            });
            translator.code.push(ControlFlow::Goto { label: end_label });

            translator.code.label(false_label);
            translator.code.push(Sequental::Assign {
                dst: result,
                value: Atom::Value(Value::Int(0)),
            });

            translator.code.label(end_label);

            result
        }

        thir::Expr::Binary { lhs, op, rhs } => {
            let mut lhs = translate_expr(*lhs, translator, idents, symbols);
            let mut rhs = translate_expr(*rhs, translator, idents, symbols);

            let result = idents.next(op.ty());

            if let thir::BinOp::Vec(VecOp::LeftMul, _) = op {
                std::mem::swap(&mut lhs, &mut rhs);
            }

            translator.code.push(Sequental::Binary {
                dst: result,
                op: op.into(),
                lhs: Atom::Id(lhs),
                rhs: Atom::Id(rhs),
            });

            result
        }

        thir::Expr::Unary {
            op: thir::UnOp::Neg(ty),
            rhs,
        } => {
            let ty = ty.into();

            let rhs = translate_expr(*rhs, translator, idents, symbols);

            let result = idents.next(ty);

            translator.code.push(Sequental::Unary {
                dst: result,
                op: UnOp::Neg(ty),
                operand: Atom::Id(rhs),
            });

            result
        }

        thir::Expr::Swizzle { lhs, swizzle } => {
            let lhs = translate_expr(*lhs, translator, idents, symbols);

            let ty = match swizzle.as_slice().len() {
                1 => Type::Real,
                2 => Type::F32x2,
                3 => Type::F32x3,
                4 => Type::F32x4,
                _ => unreachable!(),
            };

            let result = idents.next(ty);

            translator.code.push(Sequental::Unary {
                dst: result,
                op: UnOp::Swizzle(swizzle),
                operand: Atom::Id(lhs),
            });

            result
        }

        thir::Expr::Call { fun, args } => {
            let ret_ty = symbols.functions[fun].ret_ty.unwrap();

            let result = idents.next(ret_ty.into());

            translate_call(translator, idents, symbols, Some(result), fun, args);

            result
        }

        thir::Expr::Atom(atom) => {
            let value = translate_atom(atom, idents);

            let result = idents.next(value.ty());

            translator
                .code
                .push(Sequental::Assign { dst: result, value });

            result
        }
    }
}

pub fn translate_atom(atom: thir::Atom, idents: &mut BaseIdents) -> Atom {
    match atom {
        thir::Atom::Var(var) => Atom::Id(idents.get(var)),
        thir::Atom::Literal(literal) => Atom::Value(match literal.ty {
            thir::LiteralType::Complex => Value::F32x2(Vector2::new(
                0.0,
                parse_int::parse(&literal.value[0..literal.value.len() - 1]).unwrap(),
            )),
            thir::LiteralType::Real => Value::Real(parse_int::parse(literal.value).unwrap()),
            thir::LiteralType::Int => Value::Int(parse_int::parse(literal.value).unwrap()),
            thir::LiteralType::Bool => Value::Int(if literal.value == "true" { 1 } else { 0 }),
        }),
    }
}
