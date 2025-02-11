use smplc_ast::Pos;

pub struct Cursor<'source> {
    source: &'source str,
    pos: Pos,
}

impl<'source> Cursor<'source> {
    pub fn new(source: &'source str) -> Self {
        Self {
            source,
            pos: Pos::default(),
        }
    }

    pub fn current(&self) -> char {
        self.source[self.index()..].chars().next().unwrap()
    }

    pub fn next_ch(&mut self) -> char {
        let ch = self.current();

        self.pos.update(ch);

        ch
    }

    pub fn check(&self, char: char) -> bool {
        !self.is_eof() && self.current() == char
    }

    pub fn check_slice(&self, str: &str) -> bool {
        let start = self.index();
        let end = self.index() + str.len();

        if end > self.source.len() {
            return false;
        }

        self.slice(start, end) == str
    }

    pub fn slice(&self, start: usize, end: usize) -> &'source str {
        self.source.get(start..end).unwrap_or_default()
    }

    pub fn slice_from_current(&self, len: usize) -> &'source str {
        self.slice(self.pos.index(), self.pos.index() + len)
    }

    pub fn get_pos(&self) -> Pos {
        self.pos
    }

    pub fn skip(&mut self, count: usize) {
        for _ in 0..count {
            self.next_ch();
        }
    }

    pub fn index(&self) -> usize {
        self.pos.index()
    }

    pub fn is_eof(&self) -> bool {
        self.pos.index() >= self.source.len()
    }
}
