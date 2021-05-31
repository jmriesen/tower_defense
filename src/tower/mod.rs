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

pub struct Tower{
    pub reload_time:Duration,
    pub state:TurretState,
    pub angle:Option<f32>,
}

impl Tower{
    pub fn new(reload_time:Duration)->Self{
        Tower{
            reload_time,
            state:TurretState::Ready,
            angle : None,
        }
    }

    pub fn create(world:&mut World,transform:Transform){
        let sprite = {
            let sprites = world.fetch::<SpriteReasorces<Tower>>();
            sprites.get(0)
        };
        world.create_entity()
            .with(transform)
            .with(Tower::new(Duration::new(1, 0)))
            .with(sprite)
            .build();
    }
}



pub struct Bullet;
