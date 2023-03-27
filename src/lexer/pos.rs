#[derive(Debug, Clone)]
pub struct Pos {
    pub line: usize,
    pub column: usize,

    pub line_start: usize,
    pub index: usize,
}

impl Pos {
    pub fn change(&mut self, char: char) {
        match char {
            '\n' => self.new_line(),
            _ => self.column += 1,
        }

        self.index += 1;
    }

    pub fn new_line(&mut self) {
        self.line += 1;
        self.column = 1;
        self.line_start = self.index + 1;
    }
}

impl Default for Pos {
    fn default() -> Self {
        Self {
            line: 1,
            column: 1,
            line_start: 0,
            index: 0,
        }
    }
}
