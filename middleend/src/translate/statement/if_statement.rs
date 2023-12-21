use smplc_ast::IfStatement;
use smplc_ir::{Goto, Label, Unless};

use crate::translate::Translate;
use crate::{Error, Translator};

impl<'source> Translate<'source> for IfStatement<'source> {
    fn translate(self, translator: &mut Translator<'source>) -> Result<(), Error<'source>> {
        translator.ifs_count += 1;

        let end_label = Label(format!("endif{}", translator.ifs_count));

        let condition = self.condition.translate(translator)?;

        if let Some(else_body) = self.else_body {
            let else_label = Label(format!("else{}", translator.ifs_count));

            translator.code.push(Unless {
                condition,
                label: else_label.clone(),
            });

            self.then_body.translate(translator)?;

            translator.code.push(Goto {
                label: end_label.clone(),
            });

            translator.code.add_label(else_label);

            else_body.translate(translator)?;
        } else {
            translator.code.push(Unless {
                condition,
                label: end_label.clone(),
            });

            self.then_body.translate(translator)?;
        }

        translator.code.add_label(end_label);

        Ok(())
    }
}
