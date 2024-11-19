use crate::State;
use iced::widget::text;
use iced::Element;

#[derive(Debug, Clone, Copy)]
pub enum ShiniesMessage {
    StartEditShiny(usize),
    StopEditShiny,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Shinies {}

impl Shinies {
    pub fn new() -> Self {
        Self {}
    }
    pub fn view(&self, _state: &State) -> Element<ShiniesMessage> {
        text("Shinies screen").into()
    }
}
