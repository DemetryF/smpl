use std::collections::HashMap;

use smplc_ast::Pos;
use smplc_hir::VarRef;
use smplc_ir::{Code, Id, Label};

#[derive(Default)]
pub struct Translator {
    pub code: Code,
    pub variables: Variables,

    ifs_count: usize,
    whiles_count: usize,
}

#[derive(Default)]
pub struct Variables {
    data: HashMap<Pos, Id>,
    ids_count: usize,
}

impl Translator {
    pub fn next_if_labels(&mut self) -> (Label, Label) {
        let end_label = Label(format!("endif{}", self.ifs_count));
        let else_label = Label(format!("else{}", self.ifs_count));

        self.ifs_count += 1;

        (end_label, else_label)
    }

    pub fn next_while_labels(&mut self) -> (Label, Label) {
        self.whiles_count += 1;

        self.while_labels().unwrap()
    }

    pub fn while_labels(&mut self) -> Option<(Label, Label)> {
        if self.whiles_count != 0 {
            let start_label = Label(format!("while_start{}", self.whiles_count - 1));
            let end_label = Label(format!("while_end{}", self.whiles_count - 1));

            Some((start_label, end_label))
        } else {
            None
        }
    }
}

impl Variables {
    pub fn get_or_add(&mut self, var: VarRef) -> Id {
        self.data
            .get(&var.declared_at)
            .cloned()
            .unwrap_or_else(|| self.add(var))
    }

    pub fn add(&mut self, var: VarRef) -> Id {
        let id = self.next_id();

        self.data.insert(var.declared_at, id);

        id
    }

    pub fn get(&self, var: VarRef) -> Id {
        self.data[&var.declared_at]
    }

    pub fn next_id(&mut self) -> Id {
        self.ids_count += 1;

        Id::from(self.ids_count - 1)
    }
}
