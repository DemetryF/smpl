use smplc_ast::WhileStatement;
use smplc_ir::{Goto, Label, Unless};

use crate::{translate::Translate, Error, Translator};

impl<'source> Translate<'source> for WhileStatement<'source> {
    fn translate(self, translator: &mut Translator<'source>) -> Result<(), Error<'source>> {
        translator.whiles_count += 1;

        let (start_label, end_label) = while_labels(translator.whiles_count);

        translator.code.add_label(start_label.clone());

        let condition = self.condition.translate(translator)?;

        translator.code.push(Unless {
            condition,
            label: end_label.clone(),
        });

        self.body.translate(translator)?;

        translator.code.push(Goto { label: start_label });

        translator.code.add_label(end_label);

        Ok(())
    }
}

fn while_labels(whiles_count: usize) -> (Label, Label) {
    let start_label = Label(format!("while_start{whiles_count}"));
    let end_label = Label(format!("while_end{whiles_count}"));

    (start_label, end_label)
}
