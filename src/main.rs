use std::error::Error;
use std::fs;
use std::path::PathBuf;

use iced::widget::{button, column, container, row, text};
use iced::{Element, Fill, Task};

use diesel::Connection;

use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

use dirs;

use screens::{
    Counters, CountersMessage, Encounters, EncountersMessage, Hunts, HuntsMessage, ScreenType,
    Shinies, ShiniesMessage,
};

pub mod data;
pub mod hunt;
pub mod models;
pub mod schema;
pub mod shiny;

mod screens;

const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

fn run_migrations(
    connection: &mut impl MigrationHarness<diesel::sqlite::Sqlite>,
) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    connection.run_pending_migrations(MIGRATIONS)?;

    Ok(())
}

fn get_database_path() -> PathBuf {
    if let Some(dir) = dirs::data_dir() {
        [dir, "SHUtils".into(), "db.sqlite".into()].iter().collect()
    } else {
        PathBuf::from("db.sqlite")
    }
}

fn establish_db_connection() -> diesel::SqliteConnection {
    let database_url = get_database_path();
    diesel::SqliteConnection::establish(database_url.to_str().unwrap())
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url.display()))
}

fn main() -> iced::Result {
    if let Err(err) = fs::create_dir_all(get_database_path().parent().unwrap()) {
        panic!("Creating database directory failed: {}", err);
    }
    iced::application("SHUtils", State::update, State::view).run_with(|| {
        let (mut state, task) = State::new();
        if let Err(err) = run_migrations(&mut state.db_connection) {
            panic!("Database upgrade failed: {}", err)
        };
        (state, task)
    })
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
    fn view<'a>(&'a self, state: &'a State) -> Element<Message> {
        match &self {
            Screen::Counters(s) => s.view(state).map(Message::CountersMessage),
            Screen::Hunts(s) => s.view(state).map(Message::HuntsMessage),
            Screen::Shinies(s) => s.view(state).map(Message::ShiniesMessage),
            Screen::Encounters(s) => s.view(state).map(Message::EncountersMessage),
        }
    }
}

pub struct State {
    screen: Screen,
    show_menu: bool,
    pub active_hunts: Vec<hunt::Hunt>,
    pub db_connection: diesel::SqliteConnection,
    pub all_hunts: Vec<hunt::Hunt>,
    pub all_shinies: Vec<shiny::Shiny>,
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
        let mut db_connection = establish_db_connection();

        let all_hunts = hunt::Hunt::get_all(&mut db_connection).expect("Failed to load hunts!");
        let all_shinies =
            shiny::Shiny::get_all(&mut db_connection).expect("Failed to load shinies!");
        (
            Self {
                screen: Screen::Hunts(screens::Hunts::default()),
                show_menu: false,
                active_hunts: Vec::with_capacity(4),
                db_connection,
                all_hunts,
                all_shinies,
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
            self.screen.view(self)
        ]);
        if self.show_menu {
            menu().map(Message::MenuMessage)
        } else {
            content.into()
        }
    }
}
