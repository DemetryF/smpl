use crate::lexer::Operator;

pub struct PowerBinding;
impl PowerBinding {
    pub fn prefix(op: Operator) -> u8 {
        match op {
            Operator::Subtraction | Operator::Addition | Operator::Not => 17,

            _ => panic!("operator is not prefix"),
        }
    }

    pub fn infix(op: Operator) -> Option<(u8, u8)> {
        Some(match op {
            Operator::Assignment
            | Operator::AdditionAssignment
            | Operator::SubtractionAssignment
            | Operator::MultiplicationAssignment
            | Operator::DivisionAssignment => (2, 1),

            Operator::Or => (3, 4),
            Operator::And => (5, 6),

            Operator::Equal | Operator::NotEqual => (7, 8),

            Operator::GreaterOrEqual
            | Operator::Greater
            | Operator::LessOrEqual
            | Operator::Less => (9, 10),

            Operator::Subtraction | Operator::Addition => (11, 12),

            Operator::Multiplication
            | Operator::Division
            | Operator::WholeDivision
            | Operator::ModuloDivision => (13, 14),

            Operator::Exponentiation => (16, 15),

            _ => return None,
        })
    }
}
