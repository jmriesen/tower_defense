use amethyst::{
    core::{
        transform::Transform,
        timing::Time,
    },
    renderer::{SpriteRender},
    ecs::{ReadExpect,Entities,Join,WriteStorage,Read,System,SystemData},
    derive::SystemDesc,
};
use super::{
    TurretState,
    Tower,
    Bullet
};

use super::super::sprites_management::SpriteReasorces;

use super::super::movement::Movement;
#[derive(SystemDesc)]
pub struct FireingSystem ;


impl<'s> System<'s> for FireingSystem{
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Tower>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Movement>,
        WriteStorage<'s, SpriteRender>,
        WriteStorage<'s, Bullet>,
        ReadExpect<'s, SpriteReasorces<Bullet>>,
        Read<'s,Time>
    );

    fn run(&mut self, (entities, mut towers, mut transforms,mut movements, mut sprite_render,mut bullets,sprite, time)
           : Self::SystemData) {
        for (id,mut tower) in (& entities,&mut towers).join() {
            tower.state = match tower.state {
                TurretState::Ready =>{
                    if let Some(angle) = tower.angle{
                        let transform = transforms.get(id).unwrap().clone();
                        entities
                            .build_entity()
                            .with(sprite.get(0),&mut sprite_render)
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
