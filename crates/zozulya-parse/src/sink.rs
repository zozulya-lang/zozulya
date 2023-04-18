use rowan::{GreenNode, GreenNodeBuilder};

use crate::input::Input;
use crate::parser::Event;

pub struct Sink<'me> {
    pub input: Input<'me>,
    pub events: Vec<Event>,
    pub builder: GreenNodeBuilder<'static>,
}

impl<'me> Sink<'me> {
    pub fn finish(mut self) -> GreenNode {
        for index in 0..self.events.len() {
            match std::mem::replace(&mut self.events[index], Event::tombstone()) {
                Event::Start { kind } => {
                    self.builder.start_node(kind.into());
                }
                Event::Token { pos } => {
                    let pos = pos as usize;
                    self.builder.token(self.input.tokens[pos].into(), self.input.slice(pos))
                }
                Event::Finish => self.builder.finish_node(),
            }
        }

        self.builder.finish()
    }
}
