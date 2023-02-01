use crate::lexer::Operator::{self, *};

pub struct PowerBinding;
impl PowerBinding {
    pub fn prefix(op: Operator) -> u8 {
        match op {
            Subtraction | Addition | Not => 17,

            _ => panic!("operator is not prefix"),
        }
    }

    pub fn infix(op: Operator) -> Option<(u8, u8)> {
        Some(match op {
            Assignment
            | AdditionAssignment
            | SubtractionAssignment
            | MultiplicationAssignment
            | DivisionAssignment => (2, 1),

            Or => (3, 4),
            And => (5, 6),

            Exponentiation => (16, 15),
            Equal | NotEqual => (7, 8),
            Subtraction | Addition => (11, 12),
            GreaterOrEqual | Greater | LessOrEqual | Less => (9, 10),
            Multiplication | Division | WholeDivision | ModuloDivision => (13, 14),

            _ => return None,
        })
    }
}
