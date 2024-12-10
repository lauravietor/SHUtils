use diesel::prelude::*;

#[derive(Debug, Queryable, Selectable, Identifiable)]
#[diesel(table_name = crate::schema::hunts)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Hunt {
    pub id: i32,
    pub total_encounters: i32,
    pub phase_encounters: i32,
    pub phase_count: i32,
    pub start_time: Option<time::PrimitiveDateTime>,
    pub end_time: Option<time::PrimitiveDateTime>,
    pub completed: bool,
    pub version: Option<String>,
    pub method: Option<String>,
    pub place: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Queryable, Selectable, Identifiable, Associations)]
#[diesel(table_name = crate::schema::shinies)]
#[diesel(belongs_to(Hunt))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Shiny {
    pub id: i32,
    pub species: i32,
    pub total_encounters: Option<i32>,
    pub phase_encounters: Option<i32>,
    pub phase_number: Option<i32>,
    pub found_time: Option<time::PrimitiveDateTime>,
    pub notes: Option<String>,
    pub gender: Option<i32>,
    pub hunt_id: Option<i32>,
}
