pub mod aiming;
mod components;
pub mod fireing_system;
#[cfg(test)]
mod test_bullet_launching;
use crate::movement::Movement;
use amethyst::{core::transform::Transform, prelude::*};
use std::time::Duration;

use super::sprites_management::SpriteReasorces;

pub enum TurretState {
    Ready,
    CoolingDown(Duration),
}

pub struct BulletLaunching {
    pub reload_time: Duration,
    pub state: TurretState,
    pub angle: Option<f32>,
    pub numb_of_bullets: usize,
    pub spred_angle: f32,
}
// Aria of affect

impl BulletLaunching {
    pub fn new(reload_time: Duration, numb_of_bullets: usize, spred_angle: f32) -> Self {
        BulletLaunching {
            reload_time,
            state: TurretState::Ready,
            angle: None,
            numb_of_bullets,
            spred_angle,
        }
    }

    pub fn create(world: &mut World, transform: Transform) {
        let sprite = {
            let sprites = world.fetch::<SpriteReasorces<BulletLaunching>>();
            sprites.get(0)
        };
        world
            .create_entity()
            .with(transform)
            .with(BulletLaunching::new(
                Duration::new(1, 0),
                3,
                std::f32::consts::PI / 8.0,
            ))
            .with(sprite)
            .build();
    }
    fn calculate_launch_trajectories(&self) -> Option<Vec<Movement>> {
        // If there are n bullets then there are n-1 gaps between bullets.
        // Note there is a divide by zero issue if n = 1 so we handle that separately.
        let (spred_angle, delta_angle) = match self.numb_of_bullets {
            1 | 0 => (0., 0.),
            _ => (
                self.spred_angle,
                self.spred_angle / (self.numb_of_bullets - 1) as f32,
            ),
        };
        let angle = self.angle?;
        Some(
            (0..self.numb_of_bullets)
                .map(|i| -spred_angle / 2.0 + delta_angle * i as f32)
                .map(|offset| Movement {
                    speed: 10.,
                    angle: angle + offset,
                })
                .collect(),
        )
    }
}

pub struct Bullet;
