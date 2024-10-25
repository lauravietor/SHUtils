use iced::widget::{button, column, container, row, text};
use iced::Element;
use iced::Fill;
use iced::Task;
use screens::{
    Counters, CountersMessage, Encounters, EncountersMessage, Hunts, HuntsMessage, ScreenType,
    Shinies, ShiniesMessage,
};

mod hunt;
mod pokemon;
mod screens;

fn main() -> iced::Result {
    iced::application("SHUtils", State::update, State::view).run_with(State::new)
}

#[derive(Debug, Clone, Copy)]
enum Message {
    MenuMessage(MenuMessage),
    CountersMessage(CountersMessage),
    HuntsMessage(HuntsMessage),
    EncountersMessage(EncountersMessage),
    ShiniesMessage(ShiniesMessage),
}

#[derive(Debug, Clone, Copy)]
enum MenuMessage {
    Open,
    Close,
    ChangeScreen(ScreenType),
}

enum Screen {
    Counters(Counters),
    Hunts(Hunts),
    Shinies(Shinies),
    Encounters(Encounters),
}

impl Screen {
    fn view(&self) -> Element<Message> {
        match &self {
            Screen::Counters(s) => s.view().map(Message::CountersMessage),
            Screen::Hunts(s) => s.view().map(Message::HuntsMessage),
            Screen::Shinies(s) => s.view().map(Message::ShiniesMessage),
            Screen::Encounters(s) => s.view().map(Message::EncountersMessage),
        }
    }
}

struct State {
    screen: Screen,
    show_menu: bool,
    active_hunts: Vec<hunt::Hunt>,
}

fn menu<'a>() -> Element<'a, MenuMessage>
where
    MenuMessage: 'a,
{
    column![
        row![text("Menu"), button("x").on_press(MenuMessage::Close)],
        column![
            button("Hunts").on_press(MenuMessage::ChangeScreen(ScreenType::Hunts)),
            button("Shinies").on_press(MenuMessage::ChangeScreen(ScreenType::Shinies)),
            button("Counters").on_press(MenuMessage::ChangeScreen(ScreenType::Counters)),
            button("Encounters").on_press(MenuMessage::ChangeScreen(ScreenType::Encounters)),
        ]
    ]
    .height(Fill)
    .width(200)
    .into()
}

impl State {
    fn new() -> (Self, Task<Message>) {
        (
            Self {
                screen: Screen::Hunts(screens::Hunts::default()),
                show_menu: false,
                active_hunts: Vec::with_capacity(4),
            },
            Task::none(),
        )
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::MenuMessage(msg) => match msg {
                MenuMessage::Open => self.show_menu = true,
                MenuMessage::Close => self.show_menu = false,
                MenuMessage::ChangeScreen(screen_type) => match screen_type {
                    screens::ScreenType::Counters => {
                        let counters = screens::Counters::new();
                        self.screen = Screen::Counters(counters);
                        self.show_menu = false
                    }
                    screens::ScreenType::Hunts => {
                        let hunts = screens::Hunts::new();
                        self.screen = Screen::Hunts(hunts);
                        self.show_menu = false
                    }
                    screens::ScreenType::Shinies => {
                        let shinies = screens::Shinies::new();
                        self.screen = Screen::Shinies(shinies);
                        self.show_menu = false
                    }
                    screens::ScreenType::Encounters => {
                        let encounters = screens::Encounters::new();
                        self.screen = Screen::Encounters(encounters);
                        self.show_menu = false
                    }
                },
            },
            Message::CountersMessage(_msg) => (),
            Message::HuntsMessage(_msg) => (),
            Message::EncountersMessage(_msg) => (),
            Message::ShiniesMessage(_msg) => (),
        }
    }

    fn view(&self) -> Element<Message> {
        let content = container(column![
            button("Menu").on_press(Message::MenuMessage(MenuMessage::Open)),
            self.screen.view()
        ]);
        if self.show_menu {
            menu().map(Message::MenuMessage)
        } else {
            content.into()
        }
    }
}