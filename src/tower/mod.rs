use amethyst::{
    core::{
        transform::Transform,
        timing::Time,
    },
    renderer::{SpriteRender},
    ecs::{Entities,Join,Component,DenseVecStorage,WriteStorage,Read,System,SystemData},
    derive::SystemDesc,
};
use std::time::Duration;

use super::movement::Movement;
pub enum TurretState{
    Ready,
    CoolingDown(Duration),
}

pub struct Tower{
    pub sprites: Vec<SpriteRender>,
    pub reload_time:Duration,
    pub state:TurretState,
    pub angle:Option<f32>,
}

impl Tower{
    pub fn new(sprites: Vec<SpriteRender>,reload_time:Duration)->Self{
        Tower{
            sprites,
            reload_time,
            state:TurretState::Ready,
            angle : None,
        }
    }
}

impl Component for Tower{
    type Storage = DenseVecStorage<Self>;
}

pub struct Bullet;
impl Component for Bullet{
    type Storage = DenseVecStorage<Self>;
}

#[derive(SystemDesc)]
pub struct TowerSystem ;

impl<'s> System<'s> for TowerSystem {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Tower>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Movement>,
        WriteStorage<'s, SpriteRender>,
        WriteStorage<'s, Bullet>,
        Read<'s,Time>
    );

    fn run(&mut self, (entities, mut towers, mut transforms,mut movements, mut sprite_render,mut bullets,time): Self::SystemData) {
        for (id,mut tower) in (& entities,&mut towers).join() {
            tower.state = match tower.state {
                TurretState::Ready =>{
                    if let Some(angle) = tower.angle{
                        let transform = transforms.get(id).unwrap().clone();
                        //This dose work to make things smaller. but I am still not sure exactly what I want.
                        let sprite = tower.sprites[0].clone();
                        entities
                            .build_entity()
                            .with(sprite,&mut sprite_render)
                            .with(transform,&mut  transforms)
                            .with(Movement{speed:10.,angle},&mut movements)
                            .with(Bullet,&mut bullets)
                            .build();
                        TurretState::CoolingDown(tower.reload_time)
                    }else{
                        TurretState::Ready
                    }
                }
                TurretState::CoolingDown(time_left) =>{
                    if time_left < time.delta_time(){
                        TurretState::Ready
                    }else{
                        TurretState::CoolingDown(time_left-time.delta_time())
                    }
                }

            }
        }
    }
}
