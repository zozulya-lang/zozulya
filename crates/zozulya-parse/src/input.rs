mod cursor;

use self::cursor::Cursor;
use crate::syntax::Syntax;

#[derive(Default)]
pub struct Input<'me> {
    pub input: &'me str,
    pub tokens: Vec<Syntax>,
    pub spans: Vec<u32>,
}

impl<'me> Input<'me> {
    pub fn of(input: &'me str) -> Self {
        let mut builder = InputBuilder::new(input);
        let mut cursor = Cursor::new(input);

        while let Some(first_char) = cursor.shift() {
            let token = match first_char {
                _ if unicode_ident::is_xid_start(first_char) => {
                    cursor.shift_while(unicode_ident::is_xid_continue);
                    T![@ident]
                }
                _ if first_char.is_whitespace() => {
                    cursor.shift_while(|ch| ch.is_whitespace());
                    T![@whitespace]
                }
                _ if first_char.is_ascii_digit() => {
                    cursor.shift_while(|ch| ch.is_ascii_digit() || ch == '_');
                    T![@int]
                }
                ':' if cursor.shift_if('=') => T![:=],
                ':' => T![:],
                '=' => T![=],
                _ => T![@unknown],
            };

            let len = cursor.reset_len();
            builder.push(token, len);
        }

        builder.finish()
    }
}

#[derive(Default)]
struct InputBuilder<'me> {
    input: Input<'me>,
    offset: u32,
}

impl<'me> InputBuilder<'me> {
    fn new(input: &'me str) -> Self {
        Self { input: Input { input, ..<_>::default() }, offset: 0 }
    }

    fn push(&mut self, syntax: Syntax, len: u32) {
        self.input.tokens.push(syntax);
        self.input.spans.push(self.offset);

        self.offset += len;
    }

    fn finish(mut self) -> Input<'me> {
        self.push(T![@eof], 0);
        self.input
    }
}

#[cfg(test)]
mod tests {
    use expect_test::expect;

    use super::Input;

    #[test]
    fn it_works() {
        let input = Input::of("ident := 42");

        expect![[r#"
            [
                IDENT,
                WHITESPACE,
                ASSIGN,
                WHITESPACE,
                INT,
                EOF,
            ]
        "#]]
        .assert_debug_eq(&input.tokens);
    }
}
