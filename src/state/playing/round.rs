use core::time::Duration;

use crate::enemy::SpawnEvent;

use amethyst::prelude::Config;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Round {
    round: usize,
    number_grouth_rate: f32,
    health_grouth_rate: f32,
    pub time_between_rounds: Duration,
}

impl Round {
    pub fn advance(&mut self) -> SpawnEvent {
        self.round += 1;
        let growth_function = |round, exp| (round as f32).powf(exp).ceil() as usize;
        SpawnEvent {
            number: growth_function(self.round, self.number_grouth_rate),
            spacing: Duration::from_secs(1),
            health: growth_function(self.round, self.health_grouth_rate),
        }
    }
}
impl Default for Round {
    fn default() -> Self {
        Round::load("assets/round.ron").unwrap()
    }
}
