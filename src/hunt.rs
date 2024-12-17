use crate::data;
use crate::models::{Hunt as DbHunt, Shiny as DbShiny};
use crate::shiny::Shiny;

use crate::schema::hunts;

use chrono::{DateTime, Local, TimeZone};
use diesel::prelude::*;

use std::error::Error;

#[derive(Debug)]
pub struct Hunt {
    pub id: i32,
    pub target: data::Species,
    pub previous_encounters: i32,
    pub phase_encounters: i32,
    pub phase_count: i32,
    pub start_time: Option<DateTime<Local>>,
    pub end_time: Option<DateTime<Local>>,
    pub completed: bool,
    pub version: Option<String>,
    pub method: Option<String>,
    pub place: Option<String>,
    pub notes: Option<String>,
    pub shinies: Vec<Shiny>,
}

impl Hunt {
    pub fn from_db_hunt_and_shinies(db_hunt: DbHunt, db_shinies: Vec<DbShiny>) -> Self {
        Self {
            id: db_hunt.id,
            target: db_hunt.target.into(),
            previous_encounters: db_hunt.previous_encounters,
            phase_encounters: db_hunt.phase_encounters,
            phase_count: db_hunt.phase_count,
            start_time: db_hunt
                .start_time
                .map(|ndt| Local.from_local_datetime(&ndt).unwrap()),
            end_time: db_hunt
                .end_time
                .map(|ndt| Local.from_local_datetime(&ndt).unwrap()),
            completed: db_hunt.completed,
            version: db_hunt.version,
            method: db_hunt.method,
            place: db_hunt.place,
            notes: db_hunt.notes,
            shinies: db_shinies
                .into_iter()
                .map(|shiny| Shiny::from_db_shiny(shiny))
                .collect(),
        }
    }

    pub fn copy_into_db_hunt(&self) -> DbHunt {
        DbHunt {
            id: self.id,
            target: self.target.into(),
            previous_encounters: self.previous_encounters,
            phase_encounters: self.phase_encounters,
            phase_count: self.phase_count,
            start_time: self.start_time.map(|dt| dt.naive_local()),
            end_time: self.end_time.map(|dt| dt.naive_local()),
            completed: self.completed,
            version: self.version.clone(),
            method: self.method.clone(),
            place: self.place.clone(),
            notes: self.notes.clone(),
        }
    }

    pub fn get_all(db: &mut SqliteConnection) -> Result<Vec<Hunt>, Box<dyn Error + Send + Sync>> {
        let all_hunts = hunts::table.select(DbHunt::as_select()).load(db)?;
        let hunts_shinies = DbShiny::belonging_to(&all_hunts)
            .select(DbShiny::as_select())
            .load(db)?;

        let hunts_with_shinies = hunts_shinies
            .grouped_by(&all_hunts)
            .into_iter()
            .zip(all_hunts)
            .map(|(shinies, hunt)| Hunt::from_db_hunt_and_shinies(hunt, shinies))
            .collect();
        Ok(hunts_with_shinies)
    }

    pub fn get_by_id(
        hunt_id: i32,
        db: &mut SqliteConnection,
    ) -> Result<Hunt, Box<dyn Error + Send + Sync>> {
        let db_hunt = hunts::table
            .filter(hunts::dsl::id.eq(hunt_id))
            .select(DbHunt::as_select())
            .get_result(db)?;

        let hunt_shinies = DbShiny::belonging_to(&db_hunt)
            .select(DbShiny::as_select())
            .load(db)?;

        Ok(Hunt::from_db_hunt_and_shinies(db_hunt, hunt_shinies))
    }
}

impl std::fmt::Display for Hunt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} - {}",
            self.target,
            self.place.clone().unwrap_or("Inconnue".into())
        )
    }
}
