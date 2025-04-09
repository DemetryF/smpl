use std::collections::{HashMap, VecDeque};

use comet_ir::{Code, Label};

#[derive(Default)]
pub struct Translator<'source> {
    pub code: Code<'source>,

    pub labels: HashMap<Label, String>,
    labels_count: usize,

    loops: VecDeque<(Label, Label)>,
    loops_count: usize,
}

impl Translator<'_> {
    pub fn next_label(&mut self) -> Label {
        let label = Label::new(self.labels_count);

        self.labels.insert(label, format!("L{}", self.labels_count));

        self.labels_count += 1;

        label
    }

    pub fn join_loop(&mut self) -> (Label, Label) {
        self.loops_count += 1;

        let labels = (self.next_label(), self.next_label());

        self.loops.push_back(labels);

        self.loop_labels()
    }

    pub fn exit_loop(&mut self) {
        self.loops.pop_back();
    }

    pub fn loop_labels(&mut self) -> (Label, Label) {
        self.loops.back().copied().unwrap()
    }
}
