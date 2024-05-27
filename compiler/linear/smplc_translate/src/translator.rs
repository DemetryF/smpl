use std::collections::HashMap;

use smplc_ast::Pos;
use smplc_hir::{NumberType, VarRef};
use smplc_lir::{Code, Id, Label};

#[derive(Default)]
pub struct Translator {
    pub code: Code,
    pub variables: Variables,

    whiles_count: usize,
    labels_count: usize,
}

#[derive(Default)]
pub struct Variables {
    data: HashMap<Pos, Id>,
    ids_count: usize,
    pub types: HashMap<Id, NumberType>,
}

impl Translator {
    pub fn next_while_labels(&mut self) -> (Label, Label) {
        self.whiles_count += 1;

        self.while_labels().unwrap()
    }

    pub fn while_labels(&mut self) -> Option<(Label, Label)> {
        if self.whiles_count != 0 {
            let start_label = Label::new(format!("while_start{}", self.whiles_count - 1));
            let end_label = Label::new(format!("while_end{}", self.whiles_count - 1));

            Some((start_label, end_label))
        } else {
            None
        }
    }

    pub fn next_label(&mut self) -> Label {
        let label = Label::new(format!("label{}", self.labels_count));

        self.labels_count += 1;

        label
    }
}

impl Variables {
    pub fn get_or_add(&mut self, var: VarRef) -> Id {
        self.data
            .get(&var.declared_at.start())
            .cloned()
            .unwrap_or_else(|| self.add(var))
    }

    pub fn add(&mut self, var: VarRef) -> Id {
        let id = self.next_id(NumberType::for_ir(var.ty));

        self.data.insert(var.declared_at.start(), id);

        self.types.insert(id, NumberType::for_ir(var.ty));

        id
    }

    pub fn get(&self, var: VarRef) -> Id {
        self.data[&var.declared_at.start()]
    }

    pub fn next_id(&mut self, ty: NumberType) -> Id {
        self.ids_count += 1;

        let id = Id::from(self.ids_count - 1);

        self.types.insert(id, ty);

        id
    }

    pub fn ty(&self, id: Id) -> NumberType {
        self.types[&id]
    }
}
