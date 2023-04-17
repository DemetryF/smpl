use crate::{
    error::ErrorKind,
    instruction::{Call, Id, Push},
    translate::Translate,
    Error,
};

impl Translate for frontend::ast::Call {
    fn translate(self, translator: &mut crate::translator::Translator) -> Result<(), Error> {
        let function = translator.scopes.get_function(&self.id)?;

        if function.args_count != self.args.len() {
            let kind = ErrorKind::InvalidArgumentsCount {
                expected_args_count: function.args_count,
                received_args_count: self.args.len(),
                function_id: self.id.clone(),
            };

            let error = Error::new(kind, self.id.pos);

            translator.errors.push(error);
        }

        for arg in self.args {
            let value = arg.translate(translator)?;

            translator.code.push(Push { value });
        }

        let id = Id::from(self.id);

        translator.code.push(Call { result: None, id });

        Ok(())
    }
}
