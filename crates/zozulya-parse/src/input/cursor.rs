use std::str::Chars;

pub struct Cursor<'me> {
    chars: Chars<'me>,
    len: usize,
}

impl<'me> Cursor<'me> {
    pub fn new(input: &'me str) -> Self {
        Self { chars: input.chars(), len: input.len() }
    }

    pub fn peek(&self) -> Option<char> {
        self.chars.clone().next()
    }

    pub fn shift(&mut self) -> Option<char> {
        self.chars.next()
    }

    pub fn shift_if(&mut self, ch: char) -> bool {
        let is_present = self.peek().map_or(false, |peeked| ch == peeked);

        if is_present {
            self.shift();
        }

        is_present
    }

    pub fn shift_while(&mut self, f: impl Fn(char) -> bool + Copy) {
        while self.peek().map_or(false, f) {
            self.shift();
        }
    }

    pub fn reset_len(&mut self) -> u32 {
        let new_len = self.chars.as_str().len();
        let len = (self.len - new_len) as u32;
        self.len = new_len;
        len
    }
}
