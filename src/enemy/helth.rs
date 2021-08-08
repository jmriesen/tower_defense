use amethyst::ecs::{storage::DenseVecStorage, Component};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Component, Debug, Default, Deserialize, Serialize)]
pub struct Helth(usize);
impl Helth {
    pub fn new(hp: usize) -> Self {
        Helth(hp)
    }
    pub fn take_damage(&mut self, damage: usize) {
        self.0 = self.0.saturating_sub(damage);
    }
    pub fn value(&self) -> usize {
        self.0
    }
}
