use std::collections::HashMap;

use smplc_ast::Type;
use smplc_lir::{Code, Id, Label};
use smplc_thir::{NumberType, VarId};

#[derive(Default)]
pub struct Translator {
    pub code: Code,
    pub variables: Variables,

    whiles_count: usize,
    labels_count: usize,
}

#[derive(Default)]
pub struct Variables {
    data: HashMap<VarId, Id>,
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
    pub fn get_or_add(&mut self, var: VarId, ty: Type) -> Id {
        self.data
            .get(&var)
            .cloned()
            .unwrap_or_else(|| self.add(var, ty))
    }

    pub fn add(&mut self, var: VarId, ty: Type) -> Id {
        let ty = NumberType::for_ir(ty);

        let id = self.next_id(ty);

        self.data.insert(var, id);
        self.types.insert(id, ty);

        id
    }

    pub fn get(&self, var: VarId) -> Id {
        self.data[&var]
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
