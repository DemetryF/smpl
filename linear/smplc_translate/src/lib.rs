mod expr;
mod statement;
mod translator;

use smplc_hir::{Block, HIR};
use smplc_ir::{Code, CodeFunction};
use translator::Translator;

trait Translate: Sized {
    fn translate(self, translator: &mut Translator);
}

impl Translate for Block {
    fn translate(self, translator: &mut Translator) {
        self.statements
            .into_iter()
            .for_each(|stmt| stmt.translate(translator))
    }
}

pub fn translate(hir: HIR) -> Code {
    let mut translator = Translator::new(hir.variables_count);

    for function in hir.functions {
        let args = function
            .args
            .into_iter()
            .map(|var_ref| var_ref.id)
            .collect();

        translator.code.add_function(CodeFunction {
            id: function.function.id.clone(),
            args,
            ..Default::default()
        });

        function
            .statements
            .into_iter()
            .for_each(|stmt| stmt.translate(&mut translator))
    }

    translator.code
}
