use crate::hunt::Hunt;
use crate::State;
use iced::widget::text;
use iced::Element;
use std::vec::Vec;

#[derive(Debug, Clone, Copy)]
pub enum CountersMessage {
    Increment(usize),
    Decrement(usize),
    EditCounter(usize),
}

#[derive(Debug, Clone, Copy)]
struct Counter {
    hunt: Option<Hunt>,
    increment: u64,
}

impl Counter {
    pub fn view(&self, id: usize) {
        ()
    }
}

#[derive(Debug, Clone)]
pub struct Counters {
    hunts: Vec<Counter>,
}

impl Counters {
    pub fn new() -> Self {
        Self { hunts: Vec::new() }
    }
    pub fn view(&self, _state: &State) -> Element<CountersMessage> {
        text("Counters screen").into()
    }
}
