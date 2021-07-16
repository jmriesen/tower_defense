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

#[derive(Component,Default)]
pub struct EnemyFactory{
    stopwatch:Stopwatch,
    command: Option<SpawnEvent>,
}

impl EnemyFactory{
    fn update(&mut self,data:&mut EnemyDataStorage)->Option<Entity>{
        let SpawnEvent{mut number,spacing,health} = self.command?;
        if number != 0 &&
            (self.stopwatch == Stopwatch::Waiting ||
             self.stopwatch.elapsed()>spacing)
        {
            number-=1;
            self.command =
                if number != 0{
                    Some(SpawnEvent{number,spacing,health})
                }else{
                    None
                };
            self.stopwatch.reset();
            self.stopwatch.start();
            Some(self.spawn(health,data))
        }else{
            None
        }
    }

    fn set(&mut self,command:SpawnEvent){
        self.stopwatch.reset();
        self.command = Some(command);
    }
    fn spawn(&self,health:usize,(entities, movements, path_following, sprite_render, enemies, helth, enemy_sprite):&mut EnemyDataStorage)->Entity{
        entities
            .build_entity()
            .with(enemy_sprite.get(0),sprite_render)
            .with(Movement{speed:1.,angle:0.},movements)
            .with(PathFollowing,path_following)
            .with(Helth::new(health),helth)
            .with(Enemy,enemies)
            .build()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct SpawnEvent{
    pub number:usize,
    pub spacing: Duration,
    pub health:usize,
}

#[derive(SystemDesc)]
#[system_desc(name(SpawnSystemDesc))]
pub struct SpawnSystem{
    #[system_desc(event_channel_reader)]
    reader: ReaderId<SpawnEvent>,
}

impl SpawnSystem {
    pub fn new(reader: ReaderId<SpawnEvent>) -> Self {
        Self { reader}
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


