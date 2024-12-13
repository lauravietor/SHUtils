use crate::hunt::Hunt;
use crate::State;
use iced::widget::text;
use iced::Element;

const STARS_ICON: &[u8] = include_bytes!("../../assets/stars.svg");

#[derive(Debug, Clone, Copy)]
pub enum CountersMessage {
    Increment(usize),
    Decrement(usize),
    EditCounter(usize),
}

struct Counter {
    hunt: Option<Hunt>,
    increment: u64,
}

impl Counter {
    pub fn view(&self, id: usize) {
        ()
    }
}

pub struct Counters {}

impl Counters {
    pub fn new() -> Self {
        Self {}
    }
    pub fn view(&self, _state: &State) -> Element<CountersMessage> {
        text("Counters screen").into()
    }
}
