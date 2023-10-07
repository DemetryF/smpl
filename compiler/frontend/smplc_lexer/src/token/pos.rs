#[derive(Clone, Copy)]
pub struct Pos {
    line: usize,
    column: usize,
    line_start: usize,
}

impl Pos {
    pub fn line(self) -> usize {
        self.line
    }

    pub fn column(self) -> usize {
        self.column
    }

    pub fn line_start(self) -> usize {
        self.line_start
    }

    pub fn index(self) -> usize {
        self.line_start() + self.column() - 1
    }

    pub(crate) fn update(&mut self, char: char) {
        if char == '\n' {
            self.line += 1;
            self.column = 1;
            self.line_start = self.index();
        } else {
            self.column += 1;
        }
    }
}

impl Default for Pos {
    fn default() -> Self {
        Self {
            line: 1,
            column: 1,
            line_start: 0,
        }
    }
}

pub struct Posed<T> {
    pub value: T,
    pub pos: Pos,
}

impl<T> Posed<T> {
    pub fn map<U>(self, value: U) -> Posed<U> {
        Posed::<U> {
            value,
            pos: self.pos,
        }
    }
}

impl<T: Clone> Clone for Posed<T> {
    fn clone(&self) -> Self {
        Self {
            value: self.value.clone(),
            pos: self.pos,
        }
    }
}

impl<T: Copy> Copy for Posed<T> {}
