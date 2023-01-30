use crate::{
    parser::ast::{Atom, WhileStatement},
    translator::{
        instruction::{Goto, Instruction, Label, Unless},
        translate::Translate,
        Translator,
    },
};

impl Translate for WhileStatement {
    fn translate(self, translator: &mut Translator) -> Option<Atom> {
        translator.whiles_count += 1;

        let (start_label, end_label) = Self::labels(translator.whiles_count);

        translator.push(Instruction::Label(start_label.clone()));
        let cond = self.cond.translate(translator).unwrap();

        translator.push(Instruction::Unless(Unless::new(cond, end_label.clone())));

        self.body.translate(translator);

        translator.push(Instruction::Goto(Goto::new(start_label)));
        translator.push(Instruction::Label(end_label));

        None
    }
}

impl WhileStatement {
    fn labels(n: usize) -> (Label, Label) {
        let binding = n.to_string();
        let whiles_count = &binding;

        let start_label = Label(String::from("while_start") + whiles_count);
        let end_label = Label(String::from("while_end") + whiles_count);

        (start_label, end_label)
    }
}
