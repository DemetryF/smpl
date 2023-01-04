#[derive(Clone, Copy, Debug)]
pub struct Pos {
    pub line: usize,
    pub column: usize,
    pub line_begin: usize,
    pub index: usize,
}

impl Pos {
    pub fn empty() -> Pos {
        Pos {
            line: 1,
            column: 1,
            line_begin: 0,
            index: 0,
        }
    }
}
