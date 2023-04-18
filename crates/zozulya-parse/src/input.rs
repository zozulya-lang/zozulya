mod classes;
mod cursor;

use zozulya_ir::cst::ZozulyaKind::{self, *};

#[derive(Default)]
pub struct Input<'me> {
    pub text: &'me str,
    pub tokens: Vec<ZozulyaKind>,
    pub start_offsets: Vec<u32>,
}

impl<'me> Input<'me> {
    pub fn of(text: &'me str) -> Self {
        let mut builder = InputBuilder::new(text);
        let mut cursor = cursor::Cursor::new(text);

        while let Some(first_char) = cursor.shift() {
            let token = match first_char {
                _ if classes::is_ident_start(first_char) => {
                    cursor.shift_while(classes::is_ident_continue);
                    IDENT
                }
                _ if classes::is_whitespace(first_char) => {
                    cursor.shift_while(classes::is_whitespace);
                    WHITESPACE
                }
                _ if classes::is_dec_digit(first_char) => {
                    cursor.shift_while(classes::is_dec_digit);
                    INT
                }
                ':' if cursor.shift_if('=') => ASSIGN,
                ':' => COLON,
                '=' => EQ,
                _ => UNKNOWN,
            };

            let len = cursor.reset_len();
            builder.push(token, len);
        }

        builder.finish()
    }

    pub fn slice(&self, pos: usize) -> &str {
        let hi = self.start_offsets[pos + 1] as usize;
        let lo = self.start_offsets[pos] as usize;

        &self.text[lo..hi]
    }
}

#[derive(Default)]
struct InputBuilder<'me> {
    text: Input<'me>,
    offset: u32,
}

impl<'me> InputBuilder<'me> {
    fn new(text: &'me str) -> Self {
        Self { text: Input { text, ..<_>::default() }, offset: 0 }
    }

    fn push(&mut self, kind: ZozulyaKind, len: u32) {
        self.text.tokens.push(kind);
        self.text.start_offsets.push(self.offset);

        self.offset += len;
    }

    fn finish(mut self) -> Input<'me> {
        self.push(EOF, 0);
        self.text
    }
}

#[cfg(test)]
mod tests {
    use expect_test::expect;

    use super::Input;

    #[test]
    fn it_works() {
        let text = Input::of("ident := 42");

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
        .assert_debug_eq(&text.tokens);
    }
}
