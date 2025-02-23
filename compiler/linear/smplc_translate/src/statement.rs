use smplc_lir::{Assign, FunctionId, Goto, Return};
use smplc_thir::*;

use crate::expr::{atom_or_assign, translate_call, translate_expr, translate_logic_expr};
use crate::translator::Translator;
use crate::Translate;

impl Translate for Statement<'_> {
    fn translate(self, translator: &mut Translator, symbols: &Symbols) {
        match self {
            Statement::Expr(expr_stmt) => expr_stmt.translate(translator, symbols),
            Statement::If(if_stmt) => if_stmt.translate(translator, symbols),
            Statement::Return(return_stmt) => return_stmt.translate(translator, symbols),
            Statement::While(while_stmt) => while_stmt.translate(translator, symbols),

            Statement::Break => {
                let (_, end_label) = translator.while_labels().unwrap();

                translator.code.push(Goto { label: end_label })
            }

            Statement::Continue => {
                let (start_label, _) = translator.while_labels().unwrap();

                translator.code.push(Goto { label: start_label });
            }
        }
    }
}

impl Translate for IfStatement<'_> {
    fn translate(self, translator: &mut Translator, symbols: &Symbols) {
        let end_label = translator.next_label();

        if let Some(else_body) = self.else_body {
            let true_label = translator.next_label();
            let false_label = translator.next_label();

            translate_logic_expr(
                self.cond,
                translator,
                symbols,
                true_label.clone(),
                false_label.clone(),
            );

            translator.code.add_label(true_label);

            self.body.translate(translator, symbols);

            translator.code.push(Goto {
                label: end_label.clone(),
            });

            translator.code.add_label(false_label);
            else_body.translate(translator, symbols);
        } else {
            let true_label = translator.next_label();

            translate_logic_expr(
                self.cond,
                translator,
                symbols,
                true_label.clone(),
                end_label.clone(),
            );

            translator.code.add_label(true_label);
            self.body.translate(translator, symbols);
        }

        translator.code.add_label(end_label);
    }
}

impl Translate for WhileStatement<'_> {
    fn translate(self, translator: &mut Translator, symbols: &Symbols) {
        let (start_label, end_label) = translator.next_while_labels();

        translator.code.add_label(start_label.clone());

        self.body.translate(translator, symbols);

        translate_logic_expr(
            self.cond,
            translator,
            symbols,
            start_label,
            end_label.clone(),
        );

        translator.code.add_label(end_label);
    }
}

impl Translate for ReturnStatement<'_> {
    fn translate(self, translator: &mut Translator, symbols: &Symbols) {
        let value = self
            .value
            .map(|expr| translate_expr(expr, translator, symbols))
            .map(|value| atom_or_assign(translator, value));

        translator.code.push(Return { value })
    }
}

impl Translate for ExprStatement<'_> {
    fn translate(self, translator: &mut Translator, symbols: &Symbols) {
        match self {
            ExprStatement::Assign { var, rhs } => {
                let result_id = translator
                    .variables
                    .get_or_add(var, symbols.variables[var].ty);

                let rhs = translate_expr(rhs, translator, symbols);

                translator.code.push(Assign {
                    lhs: result_id,
                    rhs,
                })
            }

            ExprStatement::Expr(Expr::Call { fun, args }) => {
                let fun_data = &symbols.functions[fun];

                translate_call(
                    translator,
                    symbols,
                    FunctionId(fun_data.id.0.into()),
                    args,
                    &fun_data.args_types,
                );
            }

            _ => {}
        }
    }
}
