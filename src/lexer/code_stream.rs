use super::pos::Pos;

pub struct CodeStream {
    pub pos: Pos,
    code: String,
}

impl CodeStream {
    pub fn new(code: String) -> Self {
        Self {
            code,
            pos: Pos::empty(),
        }
    }

    pub fn current(&self) -> char {
        self.code
            .chars()
            .nth(self.pos.index)
            .expect("CodeStream::current")
    }

    pub fn check(&self, str: &str) -> bool {
        let start = self.pos.index;
        let end = start + str.len();

        if end > self.code.len() {
            return false;
        }

        &self.code[start..end] == str
    }

    pub fn skip(&mut self, count: usize) {
        for _ in 0..count {
            self.accept();
        }
    }

    pub fn accept(&mut self) -> char {
        let ch = self.current();

        self.pos.change(ch);

        ch
    }

    pub fn is_eof(&self) -> bool {
        self.pos.index >= self.code.len()
    }

    pub fn get_pos(&self) -> Pos {
        self.pos
    }
}
