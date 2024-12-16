pub mod counters;
pub mod hunts;
pub mod shinies;

pub use counters::{Counters, CountersMessage};
pub use hunts::{Hunts, HuntsAction, HuntsMessage};
pub use shinies::{Shinies, ShiniesMessage};

#[derive(Debug, Clone, Copy)]
pub enum ScreenType {
    Counters,
    Hunts,
    Shinies,
}
