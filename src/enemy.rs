use amethyst::{
    prelude::*,
    renderer::{SpriteRender},
    core::{
        transform::Transform,
        bundle::SystemBundle,
    },
    ecs::{Join,Component,DenseVecStorage,ReadStorage,DispatcherBuilder,Read,Entities,WriteStorage,System,SystemData},
    derive::SystemDesc,
    shrev::{EventChannel,ReaderId},
};
use amethyst::Error;


pub struct EnemyFactory{
    sprites: Vec<SpriteRender>,
}

struct EnemyConfig {
    sprite : SpriteRender,
    location:Transform,
    movement: Movement,
}

impl Component for EnemyFactory {
    type Storage = DenseVecStorage<Self>;
}

impl EnemyFactory{
    pub fn new(sprites: Vec<SpriteRender>)->Self{
        Self{
            sprites: sprites,

        }
    }
    fn spawn(&self,location:Transform)->EnemyConfig {
        EnemyConfig {
            sprite:self.sprites[0].clone(),
            location,
            movement:Movement{speed:1.,angle:0.},
        }
    }


}
pub struct Enemy;

impl Component for Enemy{
    type Storage = DenseVecStorage<Self>;
}
pub struct SpawnEvent;
use super::path::PathFollowing;
use super::movement::Movement;

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
        Entities<'s>,
        Read<'s, EventChannel<SpawnEvent>>,
        ReadStorage<'s, EnemyFactory>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Movement>,
        WriteStorage<'s, PathFollowing>,
        WriteStorage<'s, SpriteRender>,
        WriteStorage<'s, Enemy>,
    );

    fn run(&mut self, (entities, channel, factories, mut transforms,mut movements, mut path_following, mut sprite_render, mut enemies): Self::SystemData) {
        for _event in channel.read(&mut self.reader) {
            //extract all information I will need to build bullets.
            let parts :Vec<EnemyConfig> =
                (&factories,&mut transforms).join()
                .map(|(factory,transform)|factory.spawn(transform.clone()))
                .collect();
            //build enemies
            for config in parts{
                entities
                    .build_entity()
                    .with(config.sprite,&mut sprite_render)
                    .with(config.location,&mut transforms)
                    .with(config.movement,&mut movements)
                    .with(PathFollowing,&mut path_following)
                    .with(Enemy,&mut enemies)
                    .build();
            }
        }
    }
}

#[derive(Debug)]
pub struct MyBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for MyBundle {
    fn build(
        self,
        world: &mut World,
        builder: &mut DispatcherBuilder<'a, 'b>,
    ) -> Result<(), Error> {
        builder.add(
            SpawnSystemDesc::default().build(world),
            "spawnSystem",
            &[],
        );
        Ok(())
    }
}
