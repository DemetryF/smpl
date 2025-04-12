mod call;
mod const_eval;
mod expr;
mod idents;
mod logic;
mod statement;
mod translator;

use std::mem;

use comet_ir::{FunId, FunctionBody, LIR};
use smplc_thir::{FunData, Symbols, THIR};

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

        let id = idents.next();

        idents.constants.insert(id, value);
    }

    let bodies = functions
        .into_iter()
        .map(|fun| {
            let args = fun.args.into_iter().map(|var| idents.add(var)).collect();

            fun.body.translate(&mut translator, &mut idents, &symbols);

            let function = FunctionBody {
                args,
                code: mem::take(&mut translator.code),
            };

            let id = fun_id(&symbols.functions[fun.id]);

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

pub fn fun_id<'source>(data: &FunData<'source>) -> FunId<'source> {
    FunId::new(data.id.0, data.ret_ty.map(Into::into))
}
