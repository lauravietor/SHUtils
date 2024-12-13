use crate::data::Species;
use crate::hunt::Hunt;
use crate::theme::{card, navbar, side_view};
use crate::State;

use iced::alignment::{Horizontal, Vertical};
use iced::widget::{
    button, column, container, horizontal_space, mouse_area, responsive, row, scrollable, stack,
    svg, text, text_input, Container,
};
use iced::{Element, Length, Pixels, Size};

const COG_ICON: &[u8] = include_bytes!("../../assets/cog.svg");

#[derive(Debug, Clone)]
pub enum HuntsMessage {
    CreateHunt,
    DeleteHunt(usize),
    SelectHunt(usize),
    CloseSelectedHunt,
    StartEditHunt(usize),
    EditTarget(Species),
    EditTotalEncounterCount(String),
    EditPhaseEncounterCount(String),
    EditPhaseCount(String),
    EditCompleted(bool),
    EditVersion(String),
    EditMethod(String),
    EditPlace(String),
    EditNotes(String),
    StopEditHunt(bool),
}

#[derive(Debug, Clone, Copy)]
pub enum HuntsAction {
    None,
    CreateHunt,
    DeleteHunt(usize),
    SelectHunt(usize),
    CloseSelectedHunt,
    StartEditHunt(usize),
    StopEditHunt(bool),
}

fn make_row<'a>(
    label: &'a str,
    value: impl iced::widget::text::IntoFragment<'a>,
    text_size: impl Into<Pixels> + std::marker::Copy,
) -> iced::widget::Row<HuntsMessage> {
    row![
        text(label)
            .size(text_size)
            .width(Length::Fill)
            .align_x(Horizontal::Right),
        text(value).size(text_size).width(Length::Fill)
    ]
    .spacing(4)
}

impl Hunt {
    pub fn view_card(&self, index: usize) -> Container<HuntsMessage> {
        container(
            mouse_area(stack![
                container(
                    column![
                        row![
                            container(text("sprite here").width(100).height(100))
                                .align_right(Length::Fill),
                            column![
                                text(self.target.to_string()).size(20),
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
                                "Débutée le",
                                self.start_time
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
                    .on_press(HuntsMessage::StartEditHunt(index))
                )
                .width(Length::Fill)
                .align_x(Horizontal::Right)
            ])
            .on_press(HuntsMessage::SelectHunt(index)),
        )
        .width(Length::Fill)
        .style(card)
    }

    pub fn view_detailed(&self, index: usize) -> Container<HuntsMessage> {
        container(scrollable(
            column![
                row![
                    horizontal_space(),
                    button("Modifier").on_press(HuntsMessage::StartEditHunt(index)),
                    button("Fermer").on_press(HuntsMessage::CloseSelectedHunt)
                ]
                .spacing(8),
                container(text("sprite here").width(100).height(100))
                    .width(Length::Fill)
                    .align_x(Horizontal::Center),
                make_row("Espèce :", self.target.to_string(), 16),
                make_row("Rencontres (phase) :", self.phase_encounters, 16),
                make_row(
                    "Rencontres (total) :",
                    self.previous_encounters + self.phase_encounters,
                    16
                ),
                make_row(
                    "Phase actuelle :",
                    format!("Phase {}", self.phase_count),
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
                    "Débutée le",
                    self.start_time
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
        ))
        .width(Length::Fill)
        .height(Length::Fill)
        .style(side_view)
    }

    pub fn view_editing(&self) -> Container<HuntsMessage> {
        container(scrollable(
            column![
                row![
                    horizontal_space(),
                    button("Enregistrer").on_press(HuntsMessage::StopEditHunt(true)),
                    button("Annuler").on_press(HuntsMessage::StopEditHunt(false))
                ]
                .spacing(8),
                container(text("sprite here").width(100).height(100))
                    .width(Length::Fill)
                    .align_x(Horizontal::Center),
                make_row("Espèce :", self.target.to_string(), 16),
                row![
                    text("Rencontres (phase) :")
                        .size(16)
                        .width(Length::Fill)
                        .align_x(Horizontal::Right),
                    text_input("", &self.phase_encounters.to_string())
                        .size(16)
                        .on_input(HuntsMessage::EditPhaseEncounterCount)
                ]
                .spacing(8),
                row![
                    text("Rencontres (total) :")
                        .size(16)
                        .width(Length::Fill)
                        .align_x(Horizontal::Right),
                    text_input(
                        "",
                        &(self.phase_encounters + self.previous_encounters).to_string()
                    )
                    .size(16)
                    .on_input(HuntsMessage::EditTotalEncounterCount)
                ]
                .spacing(8),
                row![
                    text("Phase actuelle :")
                        .size(16)
                        .width(Length::Fill)
                        .align_x(Horizontal::Right),
                    text_input("", &self.phase_count.to_string())
                        .size(16)
                        .on_input(HuntsMessage::EditPhaseCount)
                ]
                .spacing(8),
                row![
                    text("Version :")
                        .size(16)
                        .width(Length::Fill)
                        .align_x(Horizontal::Right),
                    text_input("", &self.version.clone().unwrap_or("".into()))
                        .size(16)
                        .on_input(HuntsMessage::EditVersion)
                ]
                .spacing(8),
                row![
                    text("Méthode :")
                        .size(16)
                        .width(Length::Fill)
                        .align_x(Horizontal::Right),
                    text_input("", &self.method.clone().unwrap_or("".into()))
                        .size(16)
                        .on_input(HuntsMessage::EditMethod)
                ]
                .spacing(8),
                row![
                    text("Zone :")
                        .size(16)
                        .width(Length::Fill)
                        .align_x(Horizontal::Right),
                    text_input("", &self.method.clone().unwrap_or("".into()))
                        .size(16)
                        .on_input(HuntsMessage::EditMethod)
                ]
                .spacing(8),
                make_row(
                    "Débutée le",
                    self.start_time
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
        ))
        .width(Length::Fill)
        .height(Length::Fill)
        .style(side_view)
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Hunts {}

impl Hunts {
    pub fn new() -> Self {
        Self {}
    }

    pub fn update(&mut self, message: HuntsMessage) -> HuntsAction {
        match message {
            HuntsMessage::SelectHunt(id) => HuntsAction::SelectHunt(id),
            HuntsMessage::CloseSelectedHunt => HuntsAction::CloseSelectedHunt,
            HuntsMessage::StartEditHunt(id) => HuntsAction::StartEditHunt(id),
            HuntsMessage::StopEditHunt(save) => HuntsAction::StopEditHunt(save),
            _ => HuntsAction::None,
        }
    }

    pub fn view<'a>(&'a self, state: &'a State) -> Element<HuntsMessage> {
        let header = container(row![
            text("Mes recherches").size(24),
            horizontal_space(),
            button("Nouvelle recherche").on_press(HuntsMessage::CreateHunt)
        ])
        .style(navbar)
        .padding(16);

        let build_columns = |size: Size| {
            let n_columns: usize = match size.width {
                x if x < 400.0 => 1,
                x if x < 800.0 => 1,
                x if x < 1200.0 => 2,
                x if x < 1600.0 => 3,
                _ => 4,
            };
            scrollable(
                row((0..n_columns).map(|i| {
                    column((i..state.all_hunts.len()).step_by(n_columns).map(|index| {
                        state
                            .all_hunts
                            .get(index)
                            .map(|hunt| hunt.view_card(index).into())
                            .unwrap()
                    }))
                    .spacing(20)
                    .into()
                }))
                .spacing(20)
                .padding(40),
            )
            .into()
        };

        let content = if let Some(index) = state.editing_hunt_index {
            state
                .all_hunts
                .get(index)
                .map(|hunt| {
                    container(row![
                        scrollable(
                            column(
                                state
                                    .all_hunts
                                    .iter()
                                    .enumerate()
                                    .map(|(index, hunt)| hunt.view_card(index).into())
                            )
                            .spacing(20)
                            .padding(40)
                        ),
                        hunt.view_editing()
                    ])
                    .width(Length::Fill)
                })
                .unwrap_or(
                    container(responsive(build_columns))
                        .height(Length::Fill)
                        .width(Length::Fill),
                )
        } else if let Some(index) = state.selected_hunt {
            state
                .all_hunts
                .get(index)
                .map(|hunt| {
                    container(row![
                        scrollable(
                            column(
                                state
                                    .all_hunts
                                    .iter()
                                    .enumerate()
                                    .map(|(index, hunt)| hunt.view_card(index).into())
                            )
                            .spacing(20)
                            .padding(40)
                        ),
                        hunt.view_detailed(index)
                    ])
                    .width(Length::Fill)
                })
                .unwrap_or(
                    container(responsive(build_columns))
                        .height(Length::Fill)
                        .width(Length::Fill),
                )
        } else {
            container(responsive(build_columns))
                .height(Length::Fill)
                .width(Length::Fill)
        };

        column![header, content].into()
    }
}
