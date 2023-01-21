use crate::{
    parser::ast::{expr::Atom, statement::while_statement::WhileStatement},
    translator::{
        instruction::{Instruction, Label},
        translate::Translate,
        Translator,
    },
};

impl Translate for WhileStatement {
    fn translate(self, translator: &mut Translator) -> Option<Atom> {
        translator.whiles_count += 1;

        let binding = translator.whiles_count.to_string();
        let whiles_count = &binding.to_string();

        let while_start_label = Label(String::from("while_start") + whiles_count);
        let while_end_label = Label(String::from("while_end") + whiles_count);

        translator.push(Instruction::Label(while_start_label.clone()));
        let cond = self.cond.translate(translator).expect("");

        translator.push(Instruction::IfFalse {
            cond,
            to: while_end_label.clone(),
        });

        self.body.translate(translator);

        translator.push(Instruction::Goto {
            to: while_start_label,
        });

        translator.push(Instruction::Label(while_end_label));

        None
    }
}
