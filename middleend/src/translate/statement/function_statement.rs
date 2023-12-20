use smplc_ast::FunctionStatement;

use crate::{code::CodeFunction, scopes::Function, translate::Translate, Error, Translator};

impl<'source> Translate<'source> for FunctionStatement<'source> {
    fn translate(self, translator: &mut Translator<'source>) -> Result<(), Error<'source>> {
        let function = Function {
            defined_at: self.id.pos,
            args_count: self.args.len(),
        };

        let mut args = vec![];

        if let Err(error) = translator.scopes.add_function(&self.id, function) {
            translator.errors.push(error);
        }

        translator.scopes.fork();

        for arg in self.args.into_iter().rev() {
            match translator.scopes.add_variable(arg.clone()) {
                Ok(id) => args.push(id),
                Err(_) => {
                    let error = Error::duplicate_function_args(arg);
                    translator.errors.push(error)
                }
            };
        }

        let code_function = {
            let id = self.id.id.into();

            CodeFunction::new(id, args)
        };

        translator.code.add_function(code_function);

        self.body.translate(translator)?;

        Ok(())
    }
}
