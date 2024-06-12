use crate::{BasicBlock, Code, Instruction, Label};

#[derive(Default)]
pub struct CodeBuilder {
    pub blocks: Vec<BasicBlock>,
}

impl CodeBuilder {
    pub fn push_instr(&mut self, instr: Instruction) {
        if matches!(instr, Instruction::Goto(_) | Instruction::IfRel { .. }) {
            self.new_block()
        }

        self.last_block().push(instr)
    }

    pub fn push_label(&mut self, label: Label) {
        if self.last_block().is_empty() {
            self.last_block().label = Some(label);
        } else {
            self.blocks.push(BasicBlock::with_label(label));
        }
    }

    fn last_block(&mut self) -> &mut BasicBlock {
        if self.blocks.is_empty() {
            self.new_block()
        }

        self.blocks.last_mut().unwrap()
    }

    fn new_block(&mut self) {
        if self.blocks.is_empty() || !self.last_block().is_empty() {
            self.blocks.push(BasicBlock::default())
        }
    }

    pub fn into_graph(self) -> Code {
        let mut graph = Code::default();

        for block in self.blocks {
            graph.add_node(block);
        }

        let mut indices = graph.node_indices().peekable();

        while let Some(current_idx) = indices.next() {
            if let Some(dst_label) = graph[current_idx].tail_jump_dst() {
                for idx in graph.node_indices() {
                    if let Some(start_label) = &graph[idx].label {
                        if start_label == dst_label {
                            graph.add_edge(current_idx, idx, ());
                            break;
                        }
                    }
                }
            }

            if !matches!(
                graph[current_idx].instructions.last(),
                Some(Instruction::Goto(_))
            ) {
                if let Some(&next_idx) = indices.peek() {
                    graph.add_edge(current_idx, next_idx, ());
                }
            }
        }

        graph
    }
}
