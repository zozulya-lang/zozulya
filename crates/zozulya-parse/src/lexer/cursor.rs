use std::str::Chars;

pub struct Cursor<'me> {
    chars: Chars<'me>,
}

impl<'me> Cursor<'me> {
    pub fn new(input: &'me str) -> Self {
        Self { chars: input.chars() }
    }

    pub fn peek(&self) -> Option<char> {
        self.chars.clone().next()
    }

    pub fn shift(&mut self) -> Option<char> {
        self.chars.next()
    }

    pub fn shift_while(&mut self, f: impl Fn(char) -> bool + Copy) {
        while self.peek().map_or(false, f) {
            self.shift();
        }
    }
}
