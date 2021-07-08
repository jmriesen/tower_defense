use super::*;
use amethyst::{
    shrev::{EventChannel,ReaderId},
    ecs::{Entity},
    core::timing::Stopwatch,
};

use core::time::Duration;

use crate::movement::{
    path::PathFollowing,
    Movement,
};
type EnemyDataStorage<'s> = (
    Entities<'s>,
    WriteStorage<'s, Movement>,
    WriteStorage<'s, PathFollowing>,
    WriteStorage<'s, SpriteRender>,
    WriteStorage<'s, Enemy>,
    WriteStorage<'s, Helth>,
    ReadExpect<'s, SpriteReasorces<Enemy>>,
);

pub struct EnemyFactory{
    stopwatch:Stopwatch,
    command: Option<SpawnEvent>,

}
impl Default for EnemyFactory{
    fn default()->Self{
        EnemyFactory{
            stopwatch:Stopwatch::new(),
            command:None,
        }
    }
}
impl Component for EnemyFactory {
    type Storage = DenseVecStorage<Self>;
}
//TODO I need to finish and test this system.
impl EnemyFactory{
    fn update(&mut self,data:&mut EnemyDataStorage)->Option<Entity>{
        let offspring =
            if let Some(SpawnEvent{number,spacing,config}) = &mut self.command{
            if *number != 0 && self.stopwatch.elapsed()>*spacing{
                *number-=1;
                self.stopwatch.reset();
                self.stopwatch.start();
                let config = *config;
                Some(self.spawn(config,data))
            }else{
                None
            }
        }else{
            None
        };

        if let Some(SpawnEvent{number:0,..}) = &self.command{
            self.command = None;
        }
        offspring
    }
    fn set(&mut self,command:SpawnEvent){
        self.stopwatch.reset();
        self.stopwatch.start();
        self.command = Some(command);
    }
    fn spawn(&self,config:SpawnConfig,(entities, movements, path_following, sprite_render, enemies, helth, enemy_sprite):&mut EnemyDataStorage)->Entity{
        entities
            .build_entity()
            .with(enemy_sprite.get(0),sprite_render)
            .with(Movement{speed:1.,angle:0.},movements)
            .with(PathFollowing,path_following)
            .with(Helth::new(config.helth),helth)
            .with(Enemy,enemies)
            .build()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct SpawnEvent{
    pub number:usize,
    pub spacing: Duration,
    pub config:SpawnConfig,
}
#[derive(Debug, Clone, Copy)]
pub struct SpawnConfig{
    pub helth :usize,
}



#[derive(SystemDesc)]
#[system_desc(name(SpawnSystemDesc))]
pub struct SpawnSystem{
    #[system_desc(event_channel_reader)]
    reader: ReaderId<SpawnEvent>,
}

impl SpawnSystem {
        pub fn new(reader: ReaderId<SpawnEvent>) -> Self {
            Self { reader ,
            }
    }
}


impl<'s> System<'s> for SpawnSystem{
    type SystemData = (
        Read<'s, EventChannel<SpawnEvent>>,
        WriteStorage<'s, EnemyFactory>,
        WriteStorage<'s, Transform>,
        EnemyDataStorage<'s>,
            );

    fn run(&mut self, (channel, mut factories, mut transforms, mut data): Self::SystemData) {
        for event in channel.read(&mut self.reader) {
            for factory in (&mut factories).join(){
                factory.set(*event);
            }

        }
            let configs:Vec<(Option<Entity>,Transform)> = (&mut factories,&mut transforms).join()
                .map(
                    |(factory,transform)|
                    (factory.update(&mut data),transform.clone())
                ).collect();
            for (enemy,transform) in configs{
                if let Some(enemy) = enemy{
                    let _ = transforms.insert(enemy,transform);
                }
            }
    }
}


