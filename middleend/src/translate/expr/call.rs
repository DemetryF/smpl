use crate::{
    error::ErrorKind,
    instruction::{Call, Id, Param},
    translate::Translate,
    translator::Translator,
    Atom, Error,
};

fn translate_call(
    call: frontend::ast::Call,
    translator: &mut Translator,
    result: Option<Id>,
) -> Result<(), Error> {
    let function = translator.scopes.get_function(&call.id)?;

    if function.args_count != call.args.len() {
        let kind = ErrorKind::InvalidArgumentsCount {
            expected_args_count: function.args_count,
            received_args_count: call.args.len(),
            function_id: call.id.clone(),
        };

        let error = Error::new(kind, call.id.pos);

        translator.errors.push(error);
    }

    call.args
        .into_iter()
        .map(|arg| arg.translate(translator).map(Param::new))
        .collect::<Result<Vec<_>, Error>>()?
        .into_iter()
        .for_each(|param| translator.code.push(param));

    let id = Id::from(call.id);

    translator.code.push(Call { result, id });

    Ok(())
}

impl Translate for frontend::ast::Call {
    fn translate(self, translator: &mut Translator) -> Result<(), Error> {
        translate_call(self, translator, None)
    }
}

impl Translate<Atom> for frontend::ast::Call {
    fn translate(self, translator: &mut Translator) -> Result<Atom, Error> {
        let result = translator.create_temp_variable();

        translate_call(self, translator, Some(result.clone()))?;

        Ok(Atom::from(result))
    }
}
