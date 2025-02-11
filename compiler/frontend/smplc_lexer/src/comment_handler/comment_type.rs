use crate::Cursor;

pub struct CommentType {
    pub begin: &'static str,
    pub end: &'static str,
}

impl CommentType {
    pub fn is_begin(&self, cursor: &Cursor) -> bool {
        cursor.check_slice(self.begin)
    }

    fn is_end(&self, cursor: &Cursor) -> bool {
        cursor.check_slice(self.end)
    }

    pub fn try_skip(&self, cursor: &mut Cursor) {
        if !self.is_begin(cursor) {
            return;
        }

        cursor.skip(self.begin.len());

        while !self.is_end(cursor) && !cursor.is_eof() {
            cursor.next_ch();
        }

        cursor.skip(self.end.len());
    }
}
