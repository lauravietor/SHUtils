use crate::hunt::Hunt;

pub struct Counter {
    pub hunt: Option<usize>,
    pub inc: i32,
    pub count: i32,
}

impl Default for Counter {
    fn default() -> Self {
        Self {
            hunt: None,
            inc: 1,
            count: 0,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum CounterEditAction {
    SetHunt(usize),
    UnsetHunt,
    SetIncrement(i32),
    SetCount(i32),
}

impl Counter {
    pub fn perform(&mut self, action: CounterEditAction, hunt: Option<&mut Hunt>) {
        match action {
            CounterEditAction::SetHunt(index) => {
                self.hunt = Some(index);
            }
            CounterEditAction::UnsetHunt => {
                self.hunt = None;
            }
            CounterEditAction::SetIncrement(increment) => {
                self.inc = increment;
            }
            CounterEditAction::SetCount(count) => {
                self.count = count;
                if let Some(h) = hunt {
                    let difference = count - h.phase_encounters;
                    h.phase_encounters += difference;
                }
            }
        }
    }

    pub fn increment(&mut self, hunt: Option<&mut Hunt>) {
        self.count += self.inc;
        if let Some(h) = hunt {
            h.phase_encounters += self.inc;
        }
    }

    pub fn decrement(&mut self, hunt: Option<&mut Hunt>) {
        if self.count > 0 {
            self.count -= 1;
        }
        if let Some(h) = hunt {
            if h.phase_encounters > 0 {
                h.phase_encounters -= 1;
            }
        }
    }
}
