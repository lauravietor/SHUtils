use iced::widget::text;
use iced::Element;

#[derive(Debug, Clone, Copy)]
pub enum EncountersMessage {
    LookupEncounter(usize),
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Encounters {}

impl Encounters {
    pub fn new() -> Self {
        Self {}
    }
    pub fn view(&self) -> Element<EncountersMessage> {
        text("Encounters screen").into()
    }
}
