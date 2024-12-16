use chrono::{DateTime, Local, TimeZone};
use diesel::prelude::*;

use crate::schema::shinies;

use std::error::Error;

pub use crate::models::Shiny as DbShiny;

#[derive(Debug)]
pub struct Shiny {
    pub id: i32,
    pub species: i32,
    pub gender: Option<i32>,
    pub name: Option<String>,
    pub total_encounters: Option<i32>,
    pub phase_encounters: Option<i32>,
    pub phase_number: Option<i32>,
    pub found_time: Option<DateTime<Local>>,
    pub version: Option<String>,
    pub method: Option<String>,
    pub place: Option<String>,
    pub notes: Option<String>,
    pub hunt_id: Option<i32>,
}

impl Shiny {
    pub fn from_db_shiny(db_shiny: DbShiny) -> Self {
        Self {
            id: db_shiny.id,
            species: db_shiny.species,
            gender: db_shiny.gender,
            name: db_shiny.name,
            total_encounters: db_shiny.total_encounters,
            phase_encounters: db_shiny.phase_encounters,
            phase_number: db_shiny.phase_number,
            found_time: db_shiny
                .found_time
                .map(|ndt| Local.from_local_datetime(&ndt).unwrap()),
            version: db_shiny.version,
            method: db_shiny.method,
            place: db_shiny.place,
            notes: db_shiny.notes,
            hunt_id: db_shiny.hunt_id,
        }
    }
    pub fn get_all(db: &mut SqliteConnection) -> Result<Vec<Shiny>, Box<dyn Error + Send + Sync>> {
        Ok(shinies::table
            .select(DbShiny::as_select())
            .load(db)?
            .into_iter()
            .map(|shiny| Shiny::from_db_shiny(shiny))
            .collect())
    }

    pub fn get_by_id(
        shiny_id: i32,
        db: &mut SqliteConnection,
    ) -> Result<Shiny, Box<dyn Error + Send + Sync>> {
        let db_shiny = shinies::table
            .filter(shinies::dsl::id.eq(shiny_id))
            .select(DbShiny::as_select())
            .get_result(db)?;

        Ok(Shiny::from_db_shiny(db_shiny))
    }
}
