mod cursor;

use zozulya_ir::InputFile;

use self::cursor::Cursor;
use crate::syntax::Syntax::{self, *};

pub fn lex(db: &dyn crate::Db, input_file: InputFile) -> Vec<Syntax> {
    let mut tokens = Vec::new();
    let mut cursor = Cursor::new(input_file.input(db));

    while let Some(first_char) = cursor.shift() {
        let token = match first_char {
            _ if first_char.is_whitespace() => {
                cursor.shift_while(|ch| ch.is_whitespace());
                WHITESPACE
            }
            ':' => COLON,
            '=' => EQ,
            _ => UNKNOWN,
        };

        tokens.push(token);
    }

    tokens
}
