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
    pub fn view(&self) -> Element<ShiniesMessage> {
        text("Shinies screen").into()
    }
}
