#[derive(Clone, Copy, Debug)]
pub struct TokenPos {
    pub line: usize,
    pub column: usize,
    pub line_begin: usize,
    pub index: usize,
}

impl TokenPos {
    pub fn empty() -> Self {
        Self {
            line: 1,
            column: 1,
            line_begin: 0,
            index: 0,
        }
    }

    pub fn change(&mut self, char: char) {
        match char {
            '\n' => self.new_line(),
            _ => self.column += 1,
        }
    }

    pub fn new_line(&mut self) {
        self.line += 1;
        self.column = 1;
        self.line_begin = self.index;
    }
}
