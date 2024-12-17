use std::error::Error;
use std::fs;
use std::path::PathBuf;

use iced::widget::{button, center, column, container, mouse_area, opaque, row, stack, text};
use iced::{Color, Element, Fill, Task};

use diesel::{Connection, RunQueryDsl};

use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

use dirs;

use screens::counters::CountersAction;
use screens::shinies::ShiniesAction;
use screens::{
    Counters, CountersMessage, Hunts, HuntsAction, HuntsMessage, ScreenType, Shinies,
    ShiniesMessage,
};

pub mod counter;
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

#[derive(Debug, Clone)]
enum Message {
    MenuMessage(MenuMessage),
    CountersMessage(CountersMessage),
    HuntsMessage(HuntsMessage),
    ShiniesMessage(ShiniesMessage),
}

#[derive(Debug, Clone, Copy)]
enum MenuMessage {
    ChangeScreen(ScreenType),
}

enum Screen {
    Counters(Counters),
    Hunts(Hunts),
    Shinies(Shinies),
}

impl Screen {
    fn view<'a>(&'a self, state: &'a State) -> Element<Message> {
        match &self {
            Screen::Counters(s) => s.view(state).map(Message::CountersMessage),
            Screen::Hunts(s) => s.view(state).map(Message::HuntsMessage),
            Screen::Shinies(s) => s.view(state).map(Message::ShiniesMessage),
        }
    }
}

pub struct State {
    screen: Screen,
    pub active_counters: [crate::counter::Counter; 4],
    pub db_connection: diesel::SqliteConnection,
    pub all_hunts: Vec<hunt::Hunt>,
    pub all_shinies: Vec<shiny::Shiny>,
    pub selected_hunt: Option<usize>,
    pub selected_shiny: Option<usize>,
    editing_counter: Option<usize>,
    editing_hunt: Option<hunt::Hunt>,
    editing_shiny: Option<shiny::Shiny>,
}

fn menu<'a>() -> Element<'a, MenuMessage>
where
    MenuMessage: 'a,
{
    column![
        text("Menu"),
        column![
            button("Hunts").on_press(MenuMessage::ChangeScreen(ScreenType::Hunts)),
            button("Shinies").on_press(MenuMessage::ChangeScreen(ScreenType::Shinies)),
            button("Counters").on_press(MenuMessage::ChangeScreen(ScreenType::Counters)),
        ]
    ]
    .height(Fill)
    .width(200)
    .into()
}

impl State {
    fn insert_or_update_db_hunt(&mut self, index: usize) {
        if let Some(hunt) = self.all_hunts.get(index) {
            let _result = diesel::update(schema::hunts::table)
                .set(hunt.copy_into_db_hunt())
                .execute(&mut self.db_connection);
        }
    }

    fn delete_hunt(&mut self, index: usize) {}

    fn insert_or_update_db_shiny(&mut self, index: usize) {
        if let Some(shiny) = self.all_shinies.get(index) {
            let _result = diesel::update(schema::shinies::table)
                .set(shiny.copy_into_db_shiny())
                .execute(&mut self.db_connection);
        }
    }

    fn delete_shiny(&mut self, index: usize) {}

    fn new() -> (Self, Task<Message>) {
        let mut db_connection = establish_db_connection();

        let all_hunts = hunt::Hunt::get_all(&mut db_connection).expect("Failed to load hunts!");
        let all_shinies =
            shiny::Shiny::get_all(&mut db_connection).expect("Failed to load shinies!");
        (
            Self {
                screen: Screen::Counters(screens::Counters::default()),
                active_counters: Default::default(),
                db_connection,
                all_hunts,
                all_shinies,
                selected_hunt: None,
                selected_shiny: None,
                editing_counter: None,
                editing_hunt: None,
                editing_shiny: None,
            },
            Task::none(),
        )
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::MenuMessage(msg) => match msg {
                MenuMessage::ChangeScreen(screen_type) => match screen_type {
                    screens::ScreenType::Counters => {
                        let counters = screens::Counters::new();
                        self.screen = Screen::Counters(counters);
                        Task::none()
                    }
                    screens::ScreenType::Hunts => {
                        let hunts = screens::Hunts::new();
                        self.screen = Screen::Hunts(hunts);
                        Task::none()
                    }
                    screens::ScreenType::Shinies => {
                        let shinies = screens::Shinies::new();
                        self.screen = Screen::Shinies(shinies);
                        Task::none()
                    }
                },
            },
            Message::CountersMessage(msg) => {
                if let Screen::Counters(screen) = &mut self.screen {
                    let action = screen.update(msg);

                    match action {
                        CountersAction::Increment(id) => {
                            let c = &mut self.active_counters[id];
                            if let Some(index) = c.hunt {
                                c.increment(self.all_hunts.get_mut(index));
                                self.insert_or_update_db_hunt(index);
                            } else {
                                c.increment(None);
                            }
                            Task::none()
                        }
                        CountersAction::Decrement(id) => {
                            let c = &mut self.active_counters[id];
                            if let Some(index) = c.hunt {
                                c.decrement(self.all_hunts.get_mut(index));
                                self.insert_or_update_db_hunt(index);
                            } else {
                                c.decrement(None);
                            }
                            Task::none()
                        }
                        CountersAction::EditCounter(edit_action) => {
                            if let Some(id) = self.editing_counter {
                                let c = &mut self.active_counters[id];
                                if let Some(index) = c.hunt {
                                    c.perform(edit_action, self.all_hunts.get_mut(index));
                                    self.insert_or_update_db_hunt(index);
                                } else {
                                    c.perform(edit_action, None);
                                }
                            }
                            Task::none()
                        }
                        CountersAction::StartEditCounter(id) => {
                            self.editing_counter = Some(id);
                            Task::none()
                        }
                        CountersAction::StopEditCounter => {
                            self.editing_counter = None;
                            Task::none()
                        }
                        _ => Task::none(),
                    }
                } else {
                    Task::none()
                }
            }
            Message::HuntsMessage(msg) => {
                if let Screen::Hunts(screen) = &mut self.screen {
                    let action = screen.update(msg);

                    match action {
                        HuntsAction::SelectHunt(index) => {
                            self.selected_hunt = Some(index);
                        }
                        HuntsAction::CloseSelectedHunt => {
                            self.selected_hunt = None;
                        }
                        _ => {}
                    }
                    Task::none()
                } else {
                    Task::none()
                }
            }
            Message::ShiniesMessage(msg) => {
                if let Screen::Shinies(screen) = &mut self.screen {
                    let action = screen.update(msg);

                    match action {
                        ShiniesAction::SelectShiny(index) => {
                            self.selected_shiny = Some(index);
                        }
                        ShiniesAction::CloseSelectedShiny => {
                            self.selected_shiny = None;
                        }
                        _ => {}
                    }
                    Task::none()
                } else {
                    Task::none()
                }
            }
            _ => Task::none(),
        }
    }

    fn view(&self) -> Element<Message> {
        let content = container(self.screen.view(self));
        let modal: Option<Element<Message>> = self.editing_counter.map(|counter_id| {
            self.active_counters[counter_id]
                .edit_modal(counter_id, self)
                .map(Message::CountersMessage)
                .into()
        });

        match modal {
            None => row![menu().map(Message::MenuMessage), content].into(),
            Some(m) => stack![
                row![menu().map(Message::MenuMessage), content],
                opaque(
                    mouse_area(center(opaque(m)).style(|_theme| {
                        container::Style {
                            background: Some(
                                Color {
                                    a: 0.8,
                                    ..Color::BLACK
                                }
                                .into(),
                            ),
                            ..container::Style::default()
                        }
                    }))
                    .on_press(Message::CountersMessage(CountersMessage::StopEditCounter))
                )
            ]
            .into(),
        }
    }
}
