mod comment_type;

use crate::Cursor;

use self::comment_type::CommentType;

const COMMENTS: [CommentType; 2] = [
    CommentType {
        begin: "//",
        end: "\n",
    },
    CommentType {
        begin: "/*",
        end: "*/",
    },
];

pub struct CommentsHandler;
impl CommentsHandler {
    pub fn skip(cursor: &mut Cursor) {
        Self::skip_spaces(cursor);

        for comment_type in COMMENTS.iter() {
            comment_type.try_skip(cursor);
        }

        Self::skip_spaces(cursor);

        if COMMENTS.into_iter().any(|c| c.is_begin(cursor)) {
            Self::skip(cursor);
        }
    }

    pub fn skip_spaces(cursor: &mut Cursor) {
        while !cursor.is_eof() && cursor.current().is_ascii_whitespace() {
            cursor.next_ch();
        }
    }
}
