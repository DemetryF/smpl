use crate::Id;

#[derive(Clone, PartialEq)]
pub enum Atom {
    Id(Id),
    Number(f32),
}
