pub use crate::token::Pos;

pub struct CodeStream<'source> {
    pos: Pos,
    source: &'source str,
}

impl<'source> CodeStream<'source> {
    pub fn new(source: &'source str) -> Self {
        Self {
            source,
            pos: Pos::default(),
        }
    }

    /// returns char at the current position.
    pub fn current(&self) -> char {
        self.source[self.index()..].chars().next().unwrap()
    }

    /// shifts current position forward by one char and returns that char.
    pub fn next(&mut self) -> char {
        let char = self.current();

        self.pos.update(char);

        char
    }

    pub fn try_consume(&mut self, char: char) -> bool {
        if self.check(char) {
            self.next();

            true
        } else {
            false
        }
    }

    /// checks if the current character is equal to char
    pub fn check(&self, char: char) -> bool {
        if self.is_eof() {
            return false;
        }

        self.current() == char
    }

    /// check if the substring starting at the current position is equal to seq
    pub fn check_seq(&self, seq: &str) -> bool {
        let start = self.index();
        let end = self.index() + seq.len();

        if end > self.source.len() {
            return false;
        }

        self.slice(start, end) == seq
    }

    /// returns a slice of the source at position start and ending at position end
    pub fn slice(&self, start: usize, end: usize) -> &'source str {
        self.source.get(start..end).unwrap_or_default()
    }

    /// returns a slice of the source starting at the current position and with given length
    pub fn slice_from_current(&self, len: usize) -> &str {
        let start = self.index();
        let end = self.index() + len;

        self.slice(start, end)
    }

    /// shifts the position by n characters
    pub fn skip_n(&mut self, n: usize) {
        for _ in 0..n {
            self.next();
        }
    }

    /// checks if the end of the source code has been reached.
    pub fn is_eof(&self) -> bool {
        self.index() + 1 > self.source.len()
    }

    /// returns current position index
    pub fn index(&self) -> usize {
        self.pos.index()
    }

    /// returns current position
    pub fn pos(&self) -> Pos {
        self.pos
    }
}
