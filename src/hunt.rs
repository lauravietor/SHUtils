use crate::pokemon::Pokemon;

#[derive(Debug, Clone, Copy)]
pub struct Hunt {
    pub count: u64,
    pub pokemon: Pokemon,
}
