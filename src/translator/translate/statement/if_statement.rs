use crate::{
    parser::ast::{expr::Atom, statement::if_statement::IfStatement},
    translator::{
        instruction::{Instruction, Label},
        translate::Translate,
        Translator,
    },
};

impl Translate for IfStatement {
    fn translate(self, translator: &mut Translator) -> Option<Atom> {
        translator.ifs_count += 1;

        let binding = translator.ifs_count.to_string();
        let ifs_count = &binding.as_str();

        let cond = self.cond.translate(translator).expect("");
        let endif_label = Label(String::from("endif") + ifs_count);

        if let Some(else_body) = self.else_body {
            let else_label = Label(String::from("else") + ifs_count);

            translator.push(Instruction::IfFalse {
                cond,
                to: else_label.clone(),
            });

            self.then_body.translate(translator);
            translator.push(Instruction::Goto {
                to: endif_label.clone(),
            });

            translator.push(Instruction::Label(else_label));
            else_body.translate(translator);
        } else {
            translator.push(Instruction::IfFalse {
                cond,
                to: endif_label.clone(),
            });
            self.then_body.translate(translator);
        }

        translator.push(Instruction::Label(endif_label));

        None
    }
}
