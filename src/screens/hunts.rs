use crate::hunt::Hunt;
use iced::widget::text;
use iced::Element;

#[derive(Debug, Clone, Copy)]
pub enum HuntsMessage {
    CreateHunt(usize),
    DeleteHunt(usize),
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Hunts {}

impl Hunts {
    pub fn new() -> Self {
        Self {}
    }
    pub fn view(&self) -> Element<HuntsMessage> {
        text("Hunts screen").into()
    }
}
