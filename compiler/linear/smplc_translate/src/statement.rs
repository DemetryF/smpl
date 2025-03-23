use std::{
    collections::{HashMap, HashSet},
    mem,
};

use smplc_lir::{Atom, ControlFlow, Id, Phi};
use smplc_thir::{self as thir, Block};
use smplc_thir::{Symbols, VarId};

use crate::{
    call::translate_call, expr::translate_expr, idents::BaseIdents, logic::translate_logic,
    translator::Translator, Translate,
};

impl Translate for thir::Statement<'_> {
    fn translate(self, translator: &mut Translator, idents: &mut BaseIdents, symbols: &Symbols) {
        match self {
            thir::Statement::Expr(expr_statement) => {
                expr_statement.translate(translator, idents, symbols)
            }
            thir::Statement::If(if_statement) => {
                if_statement.translate(translator, idents, symbols)
            }
            thir::Statement::Return(return_statement) => {
                return_statement.translate(translator, idents, symbols)
            }
            thir::Statement::While(while_statement) => {
                while_statement.translate(translator, idents, symbols);
            }
            thir::Statement::Break => {
                let (_, end_label) = translator.loop_labels();

                translator.code.push(ControlFlow::Goto { label: end_label });
            }
            thir::Statement::Continue => {
                let (start_label, _) = translator.loop_labels();

                translator
                    .code
                    .push(ControlFlow::Goto { label: start_label });
            }
        }
    }
}

impl Translate for thir::ExprStatement<'_> {
    fn translate(self, translator: &mut Translator, idents: &mut BaseIdents, symbols: &Symbols) {
        match self {
            thir::ExprStatement::Assign { var, rhs } => {
                let result_id = translate_expr(rhs, translator, idents, symbols);

                idents.set(var, result_id);
            }

            thir::ExprStatement::Expr(thir::Expr::Call { fun, args }) => {
                translate_call(translator, idents, symbols, None, fun, args);
            }

            _ => {}
        }
    }
}

impl Translate for thir::IfStatement<'_> {
    fn translate(self, translator: &mut Translator, idents: &mut BaseIdents, symbols: &Symbols) {
        let end_label = translator.next_label();

        if let Some(else_body) = self.else_body {
            let true_label = translator.next_label();
            let false_label = translator.next_label();

            translate_logic(
                self.cond,
                translator,
                idents,
                symbols,
                true_label,
                false_label,
            );

            let mut then_idents = BaseIdents::with_parent(idents);
            let mut else_idents = BaseIdents::with_parent(idents);

            translator.code.label(true_label);
            self.body.translate(translator, &mut then_idents, symbols);

            translator.code.push(ControlFlow::Goto { label: end_label });

            translator.code.label(false_label);
            else_body.translate(translator, &mut else_idents, symbols);

            translator.code.label(end_label);

            let mut updated = HashSet::new();

            let then_idents = then_idents.variables;
            let else_idents = else_idents.variables;

            for &var in then_idents.keys().chain(else_idents.keys()) {
                if updated.contains(&var) {
                    continue;
                }

                updated.insert(var);

                match (
                    idents.try_get(var),
                    then_idents.get(&var),
                    else_idents.get(&var),
                ) {
                    (None, Some(&id), None) | (None, None, Some(&id)) => {
                        idents.set(var, id);
                    }

                    (Some(a), Some(&b), None)
                    | (Some(a), None, Some(&b))
                    | (_, Some(&a), Some(&b)) => {
                        let new_id = idents.next(a.ty());

                        translator.code.push(Phi {
                            dst: new_id,
                            branches: vec![a, b],
                        });

                        idents.set(var, new_id);
                    }

                    _ => {}
                }
            }
        } else {
            let true_label = translator.next_label();

            translate_logic(
                self.cond, translator, idents, symbols, true_label, end_label,
            );

            translator.code.label(true_label);

            let mut then_idents = BaseIdents::with_parent(idents);

            self.body.translate(translator, &mut then_idents, symbols);

            let then_idents = then_idents.variables;

            for (var, alt) in then_idents {
                if let Some(prev) = idents.try_get(var) {
                    let new = idents.next(alt.ty());

                    translator.code.push(Phi {
                        dst: new,
                        branches: vec![alt, prev],
                    });

                    idents.set(var, new);
                } else {
                    idents.set(var, alt);
                }
            }

            translator.code.label(end_label);
        }
    }
}

impl Translate for thir::ReturnStatement<'_> {
    fn translate(self, translator: &mut Translator, idents: &mut BaseIdents, symbols: &Symbols) {
        let value = self
            .value
            .map(|expr| translate_expr(expr, translator, idents, symbols))
            .map(Atom::Id);

        translator.code.push(ControlFlow::Return { value });
    }
}

impl Translate for thir::WhileStatement<'_> {
    fn translate(self, translator: &mut Translator, idents: &mut BaseIdents, symbols: &Symbols) {
        let prev_code = mem::take(&mut translator.code);

        let mut phis = HashMap::default();
        consider_phis(&self.body, idents, &mut phis);

        let (start_label, end_label) = translator.join_loop();

        let body_idents = {
            let mut idents = BaseIdents::with_parent(idents);

            for (&var, &id) in &phis {
                idents.set(var, id);
            }

            self.body.translate(translator, &mut idents, symbols);

            idents
        };

        let body = mem::replace(&mut translator.code, prev_code);

        for (&var, &id) in &phis {
            translator.code.push(Phi {
                dst: id,
                branches: vec![idents.get(var), body_idents.get(var)],
            });
        }

        for (var, id) in phis {
            idents.set(var, id);
        }

        // for (var, id) in body_idents.variables {
        //     if idents.try_get(var).is_none() {
        //         idents.set(var, id);
        //     }
        // }

        let body_start = translator.next_label();

        translator.code.label(start_label);

        translate_logic(
            self.cond, translator, idents, symbols, body_start, end_label,
        );

        translator.code.label(body_start);
        translator.code.append(body);
        translator
            .code
            .push(ControlFlow::Goto { label: start_label });
        translator.code.label(end_label);

        translator.exit_loop();
    }
}

fn consider_phis(block: &Block, idents: &mut BaseIdents, phis: &mut HashMap<VarId, Id>) {
    for stmt in &block.statements {
        match stmt {
            &thir::Statement::Expr(thir::ExprStatement::Assign { var, .. }) => {
                if phis.contains_key(&var) {
                    continue;
                }

                if let Some(id) = idents.try_get(var) {
                    let id = idents.next(id.ty());
                    phis.insert(var, id);
                }
            }

            thir::Statement::If(thir::IfStatement {
                body, else_body, ..
            }) => {
                consider_phis(body, idents, phis);

                if let Some(else_body) = else_body {
                    consider_phis(else_body, idents, phis);
                }
            }

            thir::Statement::While(thir::WhileStatement { body, .. }) => {
                consider_phis(body, idents, phis);
            }
            _ => (),
        }
    }
}

impl Translate for thir::Block<'_> {
    fn translate(self, translator: &mut Translator, idents: &mut BaseIdents, symbols: &Symbols) {
        for statement in self.statements {
            statement.translate(translator, idents, symbols);
        }
    }
}
