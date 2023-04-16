use super::Instruction;

impl std::fmt::Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Instruction::Binary(a) => a.fmt(f),
            Instruction::Unary(a) => a.fmt(f),
            Instruction::Copy(a) => a.fmt(f),
            Instruction::If(a) => a.fmt(f),
            Instruction::Unless(a) => a.fmt(f),
            Instruction::Goto(a) => a.fmt(f),
            Instruction::Call(a) => a.fmt(f),
            Instruction::Push(a) => a.fmt(f),
            Instruction::Pop(a) => a.fmt(f),
            Instruction::Return(a) => a.fmt(f),
        }
    }
}
