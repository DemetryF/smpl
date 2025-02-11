use crate::cursor::Cursor;

pub fn skip(cursor: &mut Cursor) {
    skip_whitespaces(cursor);

    while skip_comment(cursor, "//", "\n") || skip_comment(cursor, "/*", "*/") {
        skip_whitespaces(cursor);
    }
}

fn skip_whitespaces(cursor: &mut Cursor) {
    while !cursor.is_eof() && cursor.current().is_whitespace() {
        cursor.next_ch();
    }
}

fn skip_comment(cursor: &mut Cursor, start: &str, end: &str) -> bool {
    if cursor.check_slice(start) {
        cursor.skip(start.len());

        while !cursor.check_slice(end) || cursor.is_eof() {
            cursor.next_ch();
        }

        cursor.skip(end.len());

        true
    } else {
        false
    }
}
