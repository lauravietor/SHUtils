use crate::hunt::Hunt;
use crate::pokemon::{GameVersion, Pokemon};
use crate::State;
use iced::alignment::{Horizontal, Vertical};
use iced::widget::{button, column, container, row, stack, svg, text};
use iced::{Element, Length};

const COG_ICON: &[u8] = include_bytes!("../../assets/cog.svg");

#[derive(Debug, Clone, Copy)]
pub enum HuntsMessage {
    CreateHunt,
    DeleteHunt(usize),
    EditHunt(usize),
}

impl Hunt {
    pub fn view_details(&self, id: usize) -> Element<HuntsMessage> {
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
                            text("Mew").size(20),
                            text(self.count).size(24),
                            text("Phase 1").size(16)
                        ]
                        .width(Length::Fill)
                        .spacing(8)
                    ]
                    .align_y(Vertical::Center)
                    .spacing(16)
                    .padding(8),
                    column![
                        make_row("Version :", "Émeraude"),
                        make_row("Méthode :", "Fuites"),
                        make_row("Zone :", "Île Loitaine"),
                        make_row("Débutée le", "19 Juillet 2005"),
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
                .on_press(HuntsMessage::EditHunt(id))
            )
            .width(Length::Fill)
            .align_x(Horizontal::Right)
        ]
        .into()
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Hunts {}

impl Hunts {
    pub fn new() -> Self {
        Self {}
    }
    pub fn view(&self, _state: &State) -> Element<HuntsMessage> {
        Hunt {
            count: 3198,
            pokemon: Pokemon {
                species: 151,
                version: GameVersion::Emerald,
            },
        }
        .view_details(0)
    }
}
