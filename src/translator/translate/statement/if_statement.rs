use crate::{
    parser::ast::{Atom, IfStatement},
    translator::{
        instruction::{Goto, Instruction, Label, Unless},
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

            translator.push(Instruction::Unless(Unless::new(cond, else_label.clone())));

            self.then_body.translate(translator);
            translator.push(Instruction::Goto(Goto::new(endif_label.clone())));

            translator.push(Instruction::Label(else_label));
            else_body.translate(translator);
        } else {
            translator.push(Instruction::Unless(Unless::new(cond, endif_label.clone())));
            self.then_body.translate(translator);
        }

        translator.push(Instruction::Label(endif_label));

        None
    }
}
