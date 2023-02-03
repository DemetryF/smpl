use super::Pos;

#[derive(Debug)]
pub struct CodeStream<'code> {
    pos: Pos,
    code: &'code str,
}

impl<'code> CodeStream<'code> {
    pub fn new(code: &'code str) -> Self {
        Self {
            code,
            pos: Pos::empty(),
        }
    }

    pub fn current(&self) -> char {
        self.code[self.pos.index..].chars().next().unwrap()
    }

    pub fn check(&self, str: &str) -> bool {
        let start = self.pos.index;

        if start + str.len() > self.code.len() {
            return false;
        }

        self.slice(start, str.len()) == str
    }

    pub fn slice(&self, start: usize, len: usize) -> &str {
        &self.code[start..start + len]
    }

    pub fn slice_from_current(&self, len: usize) -> &str {
        &self.code[self.pos.index..self.pos.index + len]
    }

    pub fn skip(&mut self, count: usize) -> &str {
        for _ in 0..count {
            self.accept();
        }

        &self.code[self.pos.index - count..self.pos.index]
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
