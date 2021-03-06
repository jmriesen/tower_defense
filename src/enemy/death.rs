use amethyst::{
    derive::SystemDesc,
    ecs::{Entities, Join, ReadStorage, System, SystemData, Write},
};

use super::{Enemy, Helth};

use crate::player::Player;

#[derive(SystemDesc)]
pub struct DeathSystem;

impl<'s> System<'s> for DeathSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Enemy>,
        ReadStorage<'s, Helth>,
        Write<'s, Player>,
    );

    fn run(&mut self, (entities, enemys, helth, mut player): Self::SystemData) {
        for (entity, helth) in (&entities, &helth).join() {
            if helth.value() == 0 && enemys.get(entity).is_some() {
                let _ = entities.delete(entity);
                player.money += 1;
            }
        }
    }
}
