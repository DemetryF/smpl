use smplc_ir::{Code, Id, Label};

#[derive(Default)]
pub struct Translator {
    pub code: Code,

    variables_count: usize,
    ifs_count: usize,
    whiles_count: usize,
}

impl Translator {
    pub fn new(variables_count: usize) -> Self {
        Self {
            variables_count,
            ..Default::default()
        }
    }

    pub fn next_id(&mut self) -> Id {
        self.variables_count += 1;

        Id::from(self.variables_count - 1)
    }

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
