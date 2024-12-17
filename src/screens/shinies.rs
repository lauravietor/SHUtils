use crate::shiny::Shiny;
use crate::State;

use iced::alignment::{Horizontal, Vertical};
use iced::widget::{
    button, column, container, horizontal_space, mouse_area, responsive, row, scrollable, stack,
    svg, text, Container,
};
use iced::{Element, Length, Pixels, Size};

const COG_ICON: &[u8] = include_bytes!("../../assets/cog.svg");

#[derive(Debug, Clone, Copy)]
pub enum ShiniesMessage {
    CreateShiny,
    DeleteShiny(usize),
    SelectShiny(usize),
    CloseSelectedShiny,
    StartEditShiny(usize),
    StopEditShiny,
}

#[derive(Debug, Clone, Copy)]
pub enum ShiniesAction {
    None,
    CreateShiny,
    DeleteShiny(usize),
    SelectShiny(usize),
    CloseSelectedShiny,
    StartEditShiny(usize),
    StopEditShiny,
}

fn make_row<'a>(
    label: impl iced::widget::text::IntoFragment<'a>,
    value: impl iced::widget::text::IntoFragment<'a>,
    text_size: impl Into<Pixels> + std::marker::Copy,
) -> iced::widget::Row<'a, ShiniesMessage> {
    row![
        text(label)
            .size(text_size)
            .width(Length::Fill)
            .align_x(Horizontal::Right),
        text(value).size(text_size).width(Length::Fill)
    ]
    .spacing(4)
}

impl Shiny {
    pub fn view_card(&self, index: usize) -> Container<ShiniesMessage> {
        container(
            mouse_area(stack![
                container(
                    column![
                        row![
                            container(text("sprite here").width(100).height(100))
                                .align_right(Length::Fill),
                            column![
                                text(if let Some(name) = self.name.clone() {
                                    name
                                } else {
                                    self.species.to_string()
                                })
                                .size(20),
                                text(match self.phase_encounters {
                                    Some(count) => format!("{} rencontres", count),
                                    None => "??? rencontres".into(),
                                })
                                .size(24),
                                text(match self.phase_number {
                                    Some(count) => format!("Phase {}", count),
                                    None => "Phase ???".into(),
                                })
                                .size(16)
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
                                self.version.clone().unwrap_or("Inconnue".into()),
                                14
                            ),
                            make_row(
                                "Méthode :",
                                self.method.clone().unwrap_or("Inconnue".into()),
                                14
                            ),
                            make_row(
                                "Zone :",
                                self.place.clone().unwrap_or("Inconnue".into()),
                                14
                            ),
                            make_row(
                                format!(
                                    "Trouvé{} le",
                                    if Some(0) == self.gender { "e" } else { "" }
                                ),
                                self.found_time
                                    .map(|dt| dt
                                        .format_localized("%-d %B %Y", chrono::Locale::fr_FR)
                                        .to_string())
                                    .unwrap_or("Inconnue".into()),
                                14
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
                    .on_press(ShiniesMessage::StartEditShiny(index))
                )
                .width(Length::Fill)
                .align_x(Horizontal::Right)
            ])
            .on_press(ShiniesMessage::SelectShiny(index)),
        )
        .width(Length::Fill)
    }

    pub fn view_detailed(&self, index: usize) -> Container<ShiniesMessage> {
        container(
            column![
                row![
                    button("Modifier").on_press(ShiniesMessage::StartEditShiny(index)),
                    button("Fermer").on_press(ShiniesMessage::CloseSelectedShiny)
                ],
                container(text("sprite here").width(100).height(100)),
                make_row("Espèce :", self.species.to_string(), 16),
                make_row(
                    "Rencontres (phase) :",
                    self.phase_encounters
                        .map(|count| count.to_string())
                        .unwrap_or("???".into()),
                    16
                ),
                make_row(
                    "Rencontres (total) :",
                    self.total_encounters
                        .map(|count| count.to_string())
                        .unwrap_or("???".into()),
                    16
                ),
                make_row(
                    "Phase :",
                    self.phase_number
                        .map(|count| count.to_string())
                        .unwrap_or("???".into()),
                    16
                ),
                make_row(
                    "Version :",
                    self.version.clone().unwrap_or("Inconnue".into()),
                    16
                ),
                make_row(
                    "Méthode :",
                    self.method.clone().unwrap_or("Inconnue".into()),
                    16
                ),
                make_row(
                    "Zone :",
                    self.place.clone().unwrap_or("Inconnue".into()),
                    16
                ),
                make_row(
                    format!("Trouvé{} le", if Some(0) == self.gender { "e" } else { "" }),
                    self.found_time
                        .map(|dt| dt
                            .format_localized("%-d %B %Y", chrono::Locale::fr_FR)
                            .to_string())
                        .unwrap_or("Inconnue".into()),
                    16
                ),
                column![text("Notes"), text(self.notes.clone().unwrap_or("".into())),],
            ]
            .spacing(12)
            .padding(16),
        )
        .width(Length::Fill)
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Shinies {}

impl Shinies {
    pub fn new() -> Self {
        Self {}
    }

    pub fn update(&mut self, message: ShiniesMessage) -> ShiniesAction {
        match message {
            ShiniesMessage::SelectShiny(id) => ShiniesAction::SelectShiny(id),
            ShiniesMessage::CloseSelectedShiny => ShiniesAction::CloseSelectedShiny,
            _ => ShiniesAction::None,
        }
    }

    pub fn view<'a>(&'a self, state: &'a State) -> Element<ShiniesMessage> {
        let header = row![
            text("Mes shinies").size(24),
            horizontal_space(),
            button("Nouveau shiny").on_press(ShiniesMessage::CreateShiny)
        ];

        let build_columns = |size: Size| {
            let n_columns: usize = match size.width {
                x if x < 400.0 => unreachable!(),
                x if x < 800.0 => 1,
                x if x < 1200.0 => 2,
                x if x < 1600.0 => 3,
                _ => 4,
            };
            scrollable(
                row((0..n_columns).map(|i| {
                    column(
                        (i..state.all_shinies.len())
                            .step_by(n_columns)
                            .map(|index| {
                                state
                                    .all_shinies
                                    .get(index)
                                    .map(|shiny| shiny.view_card(index).into())
                                    .unwrap()
                            }),
                    )
                    .spacing(20)
                    .into()
                }))
                .spacing(20)
                .padding(40),
            )
            .into()
        };

        let content = match state.selected_shiny {
            Some(index) => state
                .all_shinies
                .get(index)
                .map(|shiny| {
                    container(row![
                        scrollable(column(
                            state
                                .all_shinies
                                .iter()
                                .enumerate()
                                .map(|(index, shiny)| shiny.view_card(index).into())
                        )),
                        scrollable(shiny.view_detailed(index))
                    ])
                    .width(Length::Fill)
                })
                .unwrap_or(
                    container(responsive(build_columns))
                        .height(Length::Fill)
                        .width(Length::Fill),
                ),
            None => container(responsive(build_columns))
                .height(Length::Fill)
                .width(Length::Fill),
        };

        column![header, content].into()
    }
}
