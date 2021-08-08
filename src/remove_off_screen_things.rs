use super::ground::Ground;
use crate::enemy::Enemy;
use crate::player::Player;
use amethyst::{
    core::transform::Transform,
    derive::SystemDesc,
    ecs::{Entities, Join, ReadExpect, ReadStorage, System, SystemData, WriteExpect},
};
#[derive(SystemDesc)]
pub struct Destry;

impl<'s> System<'s> for Destry {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Transform>,
        ReadStorage<'s, Enemy>,
        ReadExpect<'s, Ground>,
        WriteExpect<'s, Player>,
    );

    fn run(&mut self, (entities, transfroms, enemys, ground, mut player): Self::SystemData) {
        for (entity, transfrom, enemy) in (&entities, &transfroms, enemys.maybe()).join() {
            if !ground.bounds_check(transfrom.clone().into()) {
                if enemy.is_some() {
                    player.lives = player.lives.saturating_sub(1);
                }
                let _ = entities.delete(entity);
            }
        }
    }
}
