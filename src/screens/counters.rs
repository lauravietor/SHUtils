use crate::hunt::Hunt;
use iced::widget::text;
use iced::Element;
use std::vec::Vec;

#[derive(Debug, Clone, Copy)]
pub enum CountersMessage {
    Increment(usize),
    Decrement(usize),
    EditCounter(usize),
}

impl Hunt {
    pub fn view_counters(&self) -> Element<CountersMessage> {
        text("Hunt counter").into()
    }
}

#[derive(Debug, Clone, Copy)]
struct Counter {
    hunt: Option<Hunt>,
    increment: u64,
}

#[derive(Debug, Clone)]
pub struct Counters {
    hunts: Vec<Counter>,
}

impl Counters {
    pub fn new() -> Self {
        Self { hunts: Vec::new() }
    }
    pub fn view(&self) -> Element<CountersMessage> {
        text("Counters screen").into()
    }
}
