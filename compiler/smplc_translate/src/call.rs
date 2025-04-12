use comet_ir::{Atom, FunId, Id, Sequental};
use smplc_thir as thir;
use smplc_thir::Symbols;

use crate::{expr::translate_expr, idents::BaseIdents, translator::Translator};

pub fn translate_call<'source>(
    translator: &mut Translator<'source>,
    idents: &mut BaseIdents,
    symbols: &Symbols<'source>,
    dst: Option<Id>,
    fun: thir::FunId,
    args: Vec<thir::Expr>,
) {
    let fun_data = &symbols.functions[fun];

    let args = args
        .into_iter()
        .zip(&fun_data.args_types)
        .map(|(arg, &ty)| {
            let arg = translate_expr(arg, translator, idents, symbols);

            // FIXME call arg can also be a value
            (Atom::Id(arg), ty.into())
        })
        .collect();

    let fun = FunId::new(fun_data.id.0, fun_data.ret_ty.map(Into::into));

    translator.code.push(Sequental::Call { dst, fun, args })
}
