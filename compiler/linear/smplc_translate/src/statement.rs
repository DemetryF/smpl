use smplc_hir::{Expr, ExprStatement, IfStatement, ReturnStatement, Statement, WhileStatement};
use smplc_lir::{Assign, FunctionId, Goto, Return};

use crate::expr::{atom_or_assign, translate_call, translate_expr, translate_logic_expr};
use crate::translator::Translator;
use crate::Translate;

impl Translate for Statement {
    fn translate(self, translator: &mut Translator) {
        match self {
            Statement::Expr(expr_stmt) => expr_stmt.translate(translator),
            Statement::If(if_stmt) => if_stmt.translate(translator),
            Statement::Return(return_stmt) => return_stmt.translate(translator),
            Statement::While(while_stmt) => while_stmt.translate(translator),

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

impl Translate for IfStatement {
    fn translate(self, translator: &mut Translator) {
        let end_label = translator.next_label();

        if let Some(else_body) = self.else_body {
            let true_label = translator.next_label();
            let false_label = translator.next_label();

            translate_logic_expr(
                self.cond,
                translator,
                true_label.clone(),
                false_label.clone(),
            );

            translator.code.add_label(true_label);
            self.body.translate(translator);
            translator.code.push(Goto {
                label: end_label.clone(),
            });

            translator.code.add_label(false_label);
            else_body.translate(translator);
        } else {
            let true_label = translator.next_label();

            translate_logic_expr(self.cond, translator, true_label.clone(), end_label.clone());

            translator.code.add_label(true_label);
            self.body.translate(translator);
        }

        translator.code.add_label(end_label);
    }
}

impl Translate for WhileStatement {
    fn translate(self, translator: &mut Translator) {
        let (start_label, end_label) = translator.next_while_labels();

        translator.code.add_label(start_label.clone());

        self.body.translate(translator);

        translate_logic_expr(self.cond, translator, start_label, end_label.clone());

        translator.code.add_label(end_label);
    }
}

impl Translate for ReturnStatement {
    fn translate(self, translator: &mut Translator) {
        let value = self
            .value
            .map(|expr| translate_expr(expr, translator))
            .map(|value| atom_or_assign(translator, value));

        translator.code.push(Return { value })
    }
}

impl Translate for ExprStatement {
    fn translate(self, translator: &mut Translator) {
        match self {
            ExprStatement::Assign { var, rhs } => {
                let result_id = translator.variables.get_or_add(var);

                let rhs = translate_expr(rhs, translator);

                translator.code.push(Assign {
                    lhs: result_id,
                    rhs,
                })
            }

            ExprStatement::Expr(Expr::Call { fun_ref, args }) => {
                translate_call(
                    translator,
                    FunctionId(fun_ref.id.clone()),
                    args,
                    &fun_ref.args,
                );
            }

            _ => {}
        }
    }
}
