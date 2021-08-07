mod components;
pub mod fireing_system;
pub mod aiming;
use std::time::Duration;
use amethyst::{
    prelude::*,
    core::transform::Transform,
};

use super::sprites_management::{SpriteReasorces};

pub enum TurretState{
    Ready,
    CoolingDown(Duration),
}

pub struct BulletLaunching{
    pub reload_time:Duration,
    pub state:TurretState,
    pub angle:Option<f32>,
    pub numb_of_bullets: usize,
    pub spred_angle:f32,
}
// Aria of affect



impl BulletLaunching{
    pub fn new(reload_time:Duration)->Self{
        BulletLaunching{
            reload_time,
            state:TurretState::Ready,
            angle : None,
            numb_of_bullets:3,
            spred_angle : std::f32::consts::PI/8.0
        }
    }

    pub fn create(world:&mut World,transform:Transform){
        let sprite = {
            let sprites = world.fetch::<SpriteReasorces<BulletLaunching>>();
            sprites.get(0)
        };
        world.create_entity()
            .with(transform)
            .with(BulletLaunching::new(Duration::new(1, 0)))
            .with(sprite)
            .build();
    }
}



pub struct Bullet;
