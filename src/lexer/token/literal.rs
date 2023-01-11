#[derive(Clone, Debug, PartialEq)]
pub enum Literal<'code> {
    Number(&'code str, f64),
    Bool(bool),
}
