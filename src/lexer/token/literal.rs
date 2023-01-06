#[derive(Clone, Debug, PartialEq)]
pub enum Literal {
    Number(f64),
    Bool(bool),
}
