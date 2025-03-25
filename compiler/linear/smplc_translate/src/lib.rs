mod call;
mod const_eval;
mod expr;
mod idents;
mod logic;
mod statement;
mod translator;

use std::mem;

use smplc_lir::{CodeFunction, LIR};
use smplc_thir::{Symbols, THIR};

use const_eval::const_eval;
use idents::BaseIdents;
use translator::Translator;

pub fn translate(thir: THIR) -> LIR {
    let THIR {
        symbols,
        functions,
        constants,
    } = thir;

    let mut translator = Translator::default();
    let mut idents = BaseIdents::default();

    for constant in constants {
        let value = const_eval(constant.value, &idents);

        let id = idents.next(constant.ty.into());

        idents.constants.insert(id, value);
    }

    let functions = functions
        .into_iter()
        .map(|fun| {
            let args = fun
                .args
                .into_iter()
                .map(|var| idents.add(var, symbols.variables[var].ty.into()))
                .collect();

            fun.body.translate(&mut translator, &mut idents, &symbols);

            let function = CodeFunction {
                args,
                code: mem::take(&mut translator.code),
            };

            (fun.id, function)
        })
        .collect();

    let function_names = symbols
        .functions
        .into_iter()
        .map(|(id, data)| (id, data.id.0.to_owned()))
        .collect();

    LIR {
        functions,
        function_names,
        constants: idents.constants,
        labels: translator.labels,
    }
}

trait Translate {
    fn translate(self, translator: &mut Translator, idents: &mut BaseIdents, symbols: &Symbols);
}
