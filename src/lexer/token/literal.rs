#[derive(Clone, Debug, PartialEq)]
pub enum Literal {
    Number(String, f64),
    Bool(bool),
}
