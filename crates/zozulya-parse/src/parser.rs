mod expr;
mod stmt;

use rowan::{GreenNode, GreenNodeBuilder};

use crate::input::Input;
use crate::sink::Sink;
use crate::syntax::Syntax;

pub struct Parser<'me> {
    input: Input<'me>,
    events: Vec<Event>,
    pos: usize,
}

impl<'me> Parser<'me> {
    pub fn new(input: Input<'me>) -> Self {
        let mut parser = Self { input, events: Vec::new(), pos: 0 };
        parser.skip_tokens();
        parser
    }

    fn next_token(&mut self) {
        self.pos += 1;
    }

    fn skip_tokens(&mut self) {
        while self.peek().is_trivia() {
            self.next_token();
        }
    }

    pub fn parse(mut self) -> GreenNode {
        self.parse_stmt();
        let sink =
            Sink { input: self.input, events: self.events, builder: GreenNodeBuilder::new() };
        sink.finish()
    }

    fn start(&mut self) -> Marker {
        let pos = self.events.len() as u32;
        self.events.push(Event::tombstone());
        Marker { pos }
    }

    fn peek(&self) -> Syntax {
        self.input.tokens[self.pos]
    }

    fn at(&self, syntax: Syntax) -> bool {
        self.peek() == syntax
    }

    fn expect(&mut self, syntax: Syntax) {
        match self.at(syntax) {
            true => {
                self.bump();
            }
            false => unreachable!("expected {syntax:?}, found {:?}", self.peek()),
        }
    }

    fn bump(&mut self) {
        let peeked = self.peek();
        if peeked != T![@eof] {
            self.events.push(Event::Token { pos: self.pos as u32 });
            self.next_token();
        }
        self.skip_tokens();
    }
}

#[derive(Debug)]
pub enum Event {
    Start { syntax: Syntax },
    Token { pos: u32 },
    Finish,
}

impl Event {
    pub fn tombstone() -> Self {
        Self::Start { syntax: T![@tombstone] }
    }
}

struct Marker {
    pos: u32,
}

impl Marker {
    fn complete(self, parser: &mut Parser, syntax: Syntax) {
        let Self { pos } = self;
        match &mut parser.events[pos as usize] {
            Event::Start { syntax: slot, .. } => {
                *slot = syntax;
            }
            _ => unreachable!(),
        }
        parser.events.push(Event::Finish);
    }
}
