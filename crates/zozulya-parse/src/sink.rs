use rowan::{GreenNode, GreenNodeBuilder, Language};

use crate::input::Input;
use crate::parser::Event;
use crate::syntax::Zozulya;

pub struct Sink<'me> {
    pub input: Input<'me>,
    pub events: Vec<Event>,
    pub builder: GreenNodeBuilder<'static>,
}

impl<'me> Sink<'me> {
    pub fn finish(mut self) -> GreenNode {
        for index in 0..self.events.len() {
            match std::mem::replace(&mut self.events[index], Event::tombstone()) {
                Event::Start { syntax } => {
                    self.builder.start_node(Zozulya::kind_to_raw(syntax));
                }
                Event::Token { pos } => {
                    let pos = pos as usize;
                    self.builder
                        .token(Zozulya::kind_to_raw(self.input.tokens[pos]), self.input.slice(pos))
                }
                Event::Finish => self.builder.finish_node(),
            }
        }

        self.builder.finish()
    }
}
