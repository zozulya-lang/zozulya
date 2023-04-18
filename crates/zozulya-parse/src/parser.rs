mod expr;
mod stmt;

use rowan::{GreenNode, GreenNodeBuilder};
use zozulya_ir::cst::ZozulyaKind::{self, *};

use crate::input::Input;
use crate::sink::Sink;

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

    fn peek(&self) -> ZozulyaKind {
        self.input.tokens[self.pos]
    }

    fn at(&self, kind: ZozulyaKind) -> bool {
        self.peek() == kind
    }

    fn expect(&mut self, kind: ZozulyaKind) {
        match self.at(kind) {
            true => {
                self.bump();
            }
            false => unreachable!("expected {kind:?}, found {:?}", self.peek()),
        }
    }

    fn bump(&mut self) {
        let peeked = self.peek();
        if peeked != EOF {
            self.events.push(Event::Token { pos: self.pos as u32 });
            self.next_token();
        }
        self.skip_tokens();
    }
}

#[derive(Debug)]
pub enum Event {
    Start { kind: ZozulyaKind },
    Token { pos: u32 },
    Finish,
}

impl Event {
    pub fn tombstone() -> Self {
        Self::Start { kind: TOMBSTONE }
    }
}

struct Marker {
    pos: u32,
}

impl Marker {
    fn complete(self, parser: &mut Parser, kind: ZozulyaKind) {
        let Self { pos } = self;
        match &mut parser.events[pos as usize] {
            Event::Start { kind: slot, .. } => {
                *slot = kind;
            }
            _ => unreachable!(),
        }
        parser.events.push(Event::Finish);
    }
}
