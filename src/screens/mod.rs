pub mod counters;
pub mod encounters;
pub mod hunts;
pub mod shinies;

pub use counters::{Counters, CountersMessage};
pub use encounters::{Encounters, EncountersMessage};
pub use hunts::{Hunts, HuntsMessage};
pub use shinies::{Shinies, ShiniesMessage};

#[derive(Debug, Clone, Copy)]
pub enum ScreenType {
    Counters,
    Hunts,
    Shinies,
    Encounters,
}
