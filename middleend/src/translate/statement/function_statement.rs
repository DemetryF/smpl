use frontend::ast::FunctionStatement;

use crate::{
    instruction::{Atom, Label, Pop},
    scopes::Function,
    translate::Translate,
    Error, Translator,
};

impl Translate for FunctionStatement {
    fn translate(self, translator: &mut Translator) -> Result<(), Error> {
        let function = Function {
            defined_at: self.id.pos,
            args_count: self.args.len(),
        };

        translator.code.add_label(Label(self.id.id.clone()));

        if let Err(error) = translator.scopes.add_function(self.id, function) {
            translator.errors.push(error);
        }

        for arg in self.args.into_iter().rev() {
            if translator.scopes.add_variable(arg.clone()).is_err() {
                let error = Error::duplicate_function_args(arg.clone());

                translator.errors.push(error)
            };

            translator.code.push(Pop {
                value: Atom::Id(arg.id),
            })
        }

        self.body.translate(translator)?;

        Ok(())
    }
}
