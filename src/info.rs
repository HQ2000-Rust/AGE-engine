use std::time::{Duration, Instant};
//for later use
#[derive(Default, Debug)]
pub struct GameInfo {
    pub cycle_count: u128,
    //who says the game starts instantly after it's creation?
    pub start_time: Option<Instant>,
    //maybe some more stuff?
}

impl GameInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
