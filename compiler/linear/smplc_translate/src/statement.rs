use smplc_hir::{Expr, ExprStatement, IfStatement, ReturnStatement, Statement, WhileStatement};
use smplc_ir::{FunctionId, Goto, Return, Unless};

use crate::expr::{translate_call, translate_expr, translate_expr_and_write_in};
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
        let (end_label, else_label) = translator.next_if_labels();

        let cond = translate_expr(self.cond, translator);

        if let Some(else_body) = self.else_body {
            translator.code.push(Unless {
                cond,
                label: else_label.clone(),
            });

            self.body.translate(translator);

            translator.code.push(Goto {
                label: end_label.clone(),
            });

            else_body.translate(translator);
        } else {
            translator.code.push(Unless {
                cond,
                label: end_label.clone(),
            });

            self.body.translate(translator);
        }

        translator.code.add_label(end_label);
    }
}

impl Translate for WhileStatement {
    fn translate(self, translator: &mut Translator) {
        let (start_label, end_label) = translator.next_while_labels();

        translator.code.add_label(start_label.clone());

        let cond = translate_expr(self.cond, translator);

        translator.code.push(Unless {
            cond,
            label: end_label.clone(),
        });

        self.body.translate(translator);

        translator.code.push(Goto { label: start_label });

        translator.code.add_label(end_label);
    }
}

impl Translate for ReturnStatement {
    fn translate(self, translator: &mut Translator) {
        let value = self.value.map(|expr| translate_expr(expr, translator));

        translator.code.push(Return { value })
    }
}

impl Translate for ExprStatement {
    fn translate(self, translator: &mut Translator) {
        match self {
            ExprStatement::Assign { var, rhs } => {
                let result_id = translator.variables.get_or_add(var);

                translate_expr_and_write_in(rhs, translator, result_id);
            }

            ExprStatement::Expr(Expr::Call { fun_ref, args }) => {
                translate_call(translator, FunctionId(fun_ref.id.clone()), args, None);
            }

            _ => {}
        }
    }
}
