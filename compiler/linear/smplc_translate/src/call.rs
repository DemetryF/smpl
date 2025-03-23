use smplc_lir::{Atom, FunId, Id, Sequental};
use smplc_thir as thir;
use smplc_thir::Symbols;

use crate::{expr::translate_expr, idents::BaseIdents, translator::Translator};

pub fn translate_call(
    translator: &mut Translator,
    idents: &mut BaseIdents,
    symbols: &Symbols,
    dst: Option<Id>,
    fun: FunId,
    args: Vec<thir::Expr>,
) {
    let args = args
        .into_iter()
        .zip(&symbols.functions[fun].args_types)
        .map(|(arg, &ty)| {
            let arg = translate_expr(arg, translator, idents, symbols);

            // FIXME call arg can also be a number
            (Atom::Id(arg), ty.into())
        })
        .collect();

    translator.code.push(Sequental::Call { dst, fun, args })
}
