use chrono::NaiveDateTime;
use diesel::prelude::*;

#[derive(Debug, Queryable, Selectable, Identifiable, AsChangeset)]
#[diesel(table_name = crate::schema::hunts)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Hunt {
    pub id: i32,
    pub target: i32,
    pub previous_encounters: i32,
    pub phase_encounters: i32,
    pub phase_count: i32,
    pub start_time: Option<NaiveDateTime>,
    pub end_time: Option<NaiveDateTime>,
    pub completed: bool,
    pub version: Option<String>,
    pub method: Option<String>,
    pub place: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Queryable, Selectable, Identifiable, Associations, AsChangeset)]
#[diesel(table_name = crate::schema::shinies)]
#[diesel(belongs_to(Hunt))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Shiny {
    pub id: i32,
    pub species: i32,
    pub gender: Option<i32>,
    pub name: Option<String>,
    pub total_encounters: Option<i32>,
    pub phase_encounters: Option<i32>,
    pub phase_number: Option<i32>,
    pub found_time: Option<NaiveDateTime>,
    pub version: Option<String>,
    pub method: Option<String>,
    pub place: Option<String>,
    pub notes: Option<String>,
    pub hunt_id: Option<i32>,
}
