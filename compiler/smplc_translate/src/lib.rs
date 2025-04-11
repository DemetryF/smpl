mod call;
mod const_eval;
mod expr;
mod idents;
mod logic;
mod statement;
mod translator;

use std::mem;

use comet_ir::{FunId, FunctionBody, LIR};
use smplc_thir::{Symbols, THIR};

use const_eval::const_eval;
use idents::BaseIdents;
use translator::Translator;

pub fn translate<'source>(thir: THIR<'source>) -> LIR<'source> {
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

    let signatures = symbols
        .functions
        .iter()
        .map(|(_, data)| (FunId(data.id.0), data.ret_ty))
        .collect();

    let bodies = functions
        .into_iter()
        .map(|fun| {
            let args = fun
                .args
                .into_iter()
                .map(|var| idents.add(var, symbols.variables[var].ty.into()))
                .collect();

            fun.body.translate(&mut translator, &mut idents, &symbols);

            let function = FunctionBody {
                args,
                code: mem::take(&mut translator.code),
            };

            let id = FunId(symbols.functions[fun.id].id.0);

            (id, function)
        })
        .collect();

    LIR {
        bodies,
        constants: idents.constants,
        labels: translator.labels,
    }
}

trait Translate<'source> {
    fn translate(
        self,
        translator: &mut Translator<'source>,
        idents: &mut BaseIdents,
        symbols: &Symbols<'source>,
    );
}
