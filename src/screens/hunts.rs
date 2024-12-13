use crate::data::names::SPECIES_NAMES;
use crate::hunt::Hunt;
use crate::State;

use iced::alignment::{Horizontal, Vertical};
use iced::widget::{button, column, container, row, stack, svg, text};
use iced::{Element, Length};

const COG_ICON: &[u8] = include_bytes!("../../assets/cog.svg");

#[derive(Debug, Clone, Copy)]
pub enum HuntsMessage {
    CreateHunt,
    DeleteHunt(i32),
    BeginEditHunt(i32),
}

impl Hunt {
    pub fn view_card(&self) -> Element<HuntsMessage> {
        let make_row = |label, value| {
            row![
                text(label)
                    .size(14)
                    .width(Length::Fill)
                    .align_x(Horizontal::Right),
                text(value).size(14).width(Length::Fill)
            ]
            .spacing(4)
        };

        stack![
            container(
                column![
                    row![
                        container(text("sprite here").width(100).height(100))
                            .align_right(Length::Fill),
                        column![
                            text(SPECIES_NAMES[self.target as usize]).size(20),
                            text(self.phase_encounters).size(24),
                            text(format!("Phase {}", self.phase_count)).size(16)
                        ]
                        .width(Length::Fill)
                        .spacing(8)
                    ]
                    .align_y(Vertical::Center)
                    .spacing(16)
                    .padding(8),
                    column![
                        make_row(
                            "Version :",
                            self.version.clone().unwrap_or("Inconnue".into())
                        ),
                        make_row(
                            "Méthode :",
                            self.method.clone().unwrap_or("Inconnue".into())
                        ),
                        make_row("Zone :", self.place.clone().unwrap_or("Inconnue".into())),
                        make_row(
                            "Débutée le",
                            self.start_time
                                .map(|dt| dt
                                    .format_localized("%-d %B %Y", chrono::Locale::fr_FR)
                                    .to_string())
                                .unwrap_or("Inconnue".into())
                        ),
                    ]
                    .spacing(12)
                    .padding([8, 16])
                ]
                .spacing(8)
                .padding(32)
            ),
            container(
                button(
                    svg::Svg::new(svg::Handle::from_memory(COG_ICON))
                        .height(32)
                        .width(32)
                )
                .height(32)
                .width(32)
                .on_press(HuntsMessage::BeginEditHunt(self.id))
            )
            .width(Length::Fill)
            .align_x(Horizontal::Right)
        ]
        .into()
    }

    pub fn view_detailed(&self) -> Element<HuntsMessage> {
        container(text("detailed")).into()
    }

    pub fn view_editing(&self) -> Element<HuntsMessage> {
        container(text("editing")).into()
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Hunts {}

impl Hunts {
    pub fn new() -> Self {
        Self {}
    }

    pub fn view<'a>(&'a self, state: &'a State) -> Element<HuntsMessage> {
        column(state.all_hunts.iter().map(|hunt| hunt.view_card())).into()
    }
}
