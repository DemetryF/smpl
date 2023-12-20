use crate::{
    error::ErrorKind,
    instruction::{Call, Id, Param},
    translate::Translate,
    translator::Translator,
    Atom, Error,
};

fn translate_call<'source>(
    call: smplc_ast::Call<'source>,
    translator: &mut Translator<'source>,
    result: Option<Id>,
) -> Result<(), Error<'source>> {
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

impl<'source> Translate<'source> for smplc_ast::Call<'source> {
    fn translate(self, translator: &mut Translator<'source>) -> Result<(), Error<'source>> {
        translate_call(self, translator, None)
    }
}

impl<'source> Translate<'source, Atom> for smplc_ast::Call<'source> {
    fn translate(self, translator: &mut Translator<'source>) -> Result<Atom, Error<'source>> {
        let result = translator.create_temp_variable();

        translate_call(self, translator, Some(result.clone()))?;

        Ok(Atom::from(result))
    }
}
