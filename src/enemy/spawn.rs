use super::*;
use amethyst::{
    shrev::{EventChannel,ReaderId},
    ecs::{Entity}
};
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

pub struct EnemyFactory;
impl Component for EnemyFactory {
    type Storage = DenseVecStorage<Self>;
}
impl EnemyFactory{
    fn spawn(&self,(entities, movements, path_following, sprite_render, enemies, helth, enemy_sprite):&mut EnemyDataStorage)->Entity{
        entities
            .build_entity()
            .with(enemy_sprite.get(0),sprite_render)
            .with(Movement{speed:1.,angle:0.},movements)
            .with(PathFollowing,path_following)
            .with(Helth::new(5),helth)
            .with(Enemy,enemies)
            .build()
    }
}
pub struct SpawnEvent;


#[derive(SystemDesc)]
#[system_desc(name(SpawnSystemDesc))]
pub struct SpawnSystem{
    #[system_desc(event_channel_reader)]
    reader: ReaderId<SpawnEvent>,
}

impl SpawnSystem {
        pub fn new(reader: ReaderId<SpawnEvent>) -> Self {
        Self { reader }
    }
}


impl<'s> System<'s> for SpawnSystem{
    type SystemData = (
        Read<'s, EventChannel<SpawnEvent>>,
        ReadStorage<'s, EnemyFactory>,
        WriteStorage<'s, Transform>,
        EnemyDataStorage<'s>,
            );

    fn run(&mut self, (channel, factories, mut transforms, mut data): Self::SystemData) {
        for _event in channel.read(&mut self.reader) {
            let configs:Vec<(Entity,Transform)> = (&factories,&mut transforms).join()
                .map(
                    |(factory,transform)|
                    (factory.spawn(&mut data),transform.clone())
                ).collect();
            for (enemy,transform) in configs{
                let _ = transforms.insert(enemy,transform);
            }
        }
    }
}


