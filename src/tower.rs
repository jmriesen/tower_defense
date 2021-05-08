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
}
impl Tower{
    pub fn new(sprites: Vec<SpriteRender>,reload_time:Duration)->Self{
        Tower{
            sprites,
            reload_time,
            state:TurretState::Ready,
        }
    }
}
impl Component for Tower{
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
        Read<'s,Time>
    );

    fn run(&mut self, (entities, mut towers, mut transforms,mut movements, mut sprite_render,time): Self::SystemData) {
        for (id,mut tower) in (& entities,&mut towers).join() {
            tower.state = match tower.state {
                TurretState::Ready =>{
                    let transform = transforms.get(id).unwrap();
                    let sprite = tower.sprites[1].clone();
                    entities
                        .build_entity()
                        .with(sprite,&mut sprite_render)
                        .with(transform.clone(),&mut  transforms)
                        .with(Movement{speed:8.,angle:0.},&mut movements)
                        .build();
                    TurretState::CoolingDown(tower.reload_time)
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
