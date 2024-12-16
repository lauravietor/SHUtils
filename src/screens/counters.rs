use crate::counter::{Counter, CounterEditAction};
use crate::State;
use iced::alignment::Horizontal;
use iced::widget::{button, column, container, row, stack, svg, text, Container};
use iced::{Element, Length};

const COG_ICON: &[u8] = include_bytes!("../../assets/cog.svg");
const STARS_ICON: &[u8] = include_bytes!("../../assets/stars.svg");

#[derive(Debug, Clone, Copy)]
pub enum CountersMessage {
    Increment(usize),
    Decrement(usize),
    StartEditCounter(usize),
    StopEditCounter,
    ShinyFound(usize),
}

#[derive(Debug, Clone, Copy)]
pub enum CountersAction {
    None,
    Increment(usize),
    Decrement(usize),
    StartEditCounter(usize),
    EditCounter(usize, CounterEditAction),
    StopEditCounter,
    ShinyFound(usize),
}

impl Counter {
    pub fn view(&self, id: usize, state: &State) -> Container<CountersMessage> {
        let count_display = match self.hunt {
            Some(index) => {
                if let Some(hunt) = state.all_hunts.get(index) {
                    container(text(hunt.phase_encounters).size(48)).center(Length::Fill)
                } else {
                    container(text(self.count).size(48)).center(Length::Fill)
                }
            }
            None => container(text(self.count).size(48)).center(Length::Fill),
        };

        container(stack![
            container(
                column![
                    count_display,
                    container(row![
                        container(
                            button(container(text("-1")).center(100))
                                .on_press(CountersMessage::Decrement(id))
                                .padding(0)
                        )
                        .center_x(Length::Fill),
                        container(
                            button(container(text(format!("+{}", self.inc))).center(100))
                                .on_press(CountersMessage::Increment(id))
                                .padding(0)
                        )
                        .center_x(Length::Fill),
                        container(
                            button(
                                container(
                                    svg::Svg::new(svg::Handle::from_memory(STARS_ICON))
                                        .height(64)
                                        .width(64)
                                )
                                .center(100)
                            )
                            .on_press(CountersMessage::ShinyFound(id))
                            .padding(0)
                        )
                        .center_x(Length::Fill)
                    ])
                    .center_y(Length::Fill)
                ]
                .spacing(8)
            ),
            container(
                button(
                    svg::Svg::new(svg::Handle::from_memory(COG_ICON))
                        .height(32)
                        .width(32)
                )
                .height(32)
                .width(32)
                .on_press(CountersMessage::StartEditCounter(id))
            )
            .width(Length::Fill)
            .align_x(Horizontal::Right)
        ])
        .padding(16)
    }

    pub fn edit_modal(&self, id: usize, _state: &State) -> Element<CountersMessage> {
        container(text(format!("Editing counter {id}"))).into()
    }
}

#[derive(Default)]
pub struct Counters {}

impl Counters {
    pub fn new() -> Self {
        Self {}
    }

    pub fn update(&mut self, message: CountersMessage) -> CountersAction {
        match message {
            CountersMessage::Increment(id) => CountersAction::Increment(id),
            CountersMessage::Decrement(id) => CountersAction::Decrement(id),
            _ => CountersAction::None,
        }
    }

    pub fn view<'a>(&'a self, state: &'a State) -> Element<CountersMessage> {
        column![
            row![
                state.active_counters[0].view(0, state).center(Length::Fill),
                state.active_counters[1].view(1, state).center(Length::Fill)
            ]
            .spacing(24),
            row![
                state.active_counters[2].view(2, state).center(Length::Fill),
                state.active_counters[3].view(3, state).center(Length::Fill)
            ]
            .spacing(24)
        ]
        .spacing(24)
        .padding(40)
        .into()
    }
}
