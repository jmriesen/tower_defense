use super::{Bullet, BulletLaunching, TurretState};
use amethyst::{
    core::{timing::Time, transform::Transform},
    derive::SystemDesc,
    ecs::{Entities, Join, Read, ReadExpect, System, SystemData, WriteStorage},
    renderer::SpriteRender,
};
use std::time::Duration;

use super::super::sprites_management::SpriteReasorces;

use super::super::movement::Movement;
#[derive(SystemDesc)]
pub struct FireingSystem;

impl<'s> System<'s> for FireingSystem {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, BulletLaunching>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Movement>,
        WriteStorage<'s, SpriteRender>,
        WriteStorage<'s, Bullet>,
        ReadExpect<'s, SpriteReasorces<Bullet>>,
        Read<'s, Time>,
    );

    fn run(
        &mut self,
        (
            entities,
            mut towers,
            mut transforms,
            mut movements,
            mut sprite_render,
            mut bullets,
            sprite,
            time,
        ): Self::SystemData,
    ) {
        for (id, mut tower) in (&entities, &mut towers).join() {
            tower.state = match tower.state {
                TurretState::Ready => {
                    if let Some(angle) = tower.angle {
                        for i in 0..tower.numb_of_bullets {
                            let offset = match tower.numb_of_bullets {
                                1 | 0 => 0.0,
                                _ => {
                                    let delta_angle =
                                        tower.spred_angle / (tower.numb_of_bullets - 1) as f32;
                                    -tower.spred_angle / 2.0 + delta_angle * i as f32
                                }
                            };
                            let transform = transforms.get(id).unwrap().clone();
                            entities
                                .build_entity()
                                .with(sprite.get(0), &mut sprite_render)
                                .with(transform, &mut transforms)
                                .with(
                                    Movement {
                                        speed: 10.,
                                        angle: angle + offset,
                                    },
                                    &mut movements,
                                )
                                .with(Bullet, &mut bullets)
                                .build();
                        }
                        TurretState::CoolingDown(tower.reload_time)
                    } else {
                        TurretState::Ready
                    }
                }
                TurretState::CoolingDown(time_left) => {
                    if time_left < time.delta_time() {
                        TurretState::Ready
                    } else {
                        TurretState::CoolingDown(time_left - time.delta_time())
                    }
                }
            }
        }

        // Set muzzle flash
        for (id, tower, mut sprite) in (&entities, &towers, &mut sprite_render).join() {
            if let TurretState::CoolingDown(time_left) = tower.state {
                if tower.reload_time - time_left < Duration::from_millis(50) {
                    sprite.sprite_number = 1
                } else if tower.reload_time - time_left < Duration::from_millis(200) {
                    sprite.sprite_number = 2
                } else {
                    sprite.sprite_number = 0
                }
            }
        }
    }
}
