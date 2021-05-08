use std::sync::Arc;
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
    path : Arc<super::path::Path>,
}

struct EnemyConfig {
    sprite : SpriteRender,
    location:Transform,
    movement: Movement,
    path_data: PathFollowing
}

impl Component for EnemyFactory {
    type Storage = DenseVecStorage<Self>;
}

impl EnemyFactory{
    //TODO this code is bloated and path should not be made hear.
    pub fn new(sprites: Vec<SpriteRender>)->Self{
        let mut path = super::path::Path::new(Transform::default());

        let mut target = Transform::default();
        target.set_translation_xyz(400.,0.,0.);
        path.add(target);

        let mut target = Transform::default();
        target.set_translation_xyz(400.,400.,0.);
        path.add(target);

        path.add(Transform::default());

        let path = Arc::new(path);
        Self{
            sprites: sprites,
            path:path,

        }
    }
    fn spawn(&self,location:Transform)->EnemyConfig {
        EnemyConfig {
            sprite:self.sprites[0].clone(),
            location,
            movement:Movement{speed:1.,angle:0.},
            path_data:PathFollowing::new(self.path.clone()),
        }
    }


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
    );

    fn run(&mut self, (entities, channel, factories, mut transforms,mut movements, mut path_following, mut sprite_render): Self::SystemData) {
        for _event in channel.read(&mut self.reader) {
            //extract all information I will need to build bullets.
            let parts :Vec<EnemyConfig> =
                (&factories,&mut transforms).join()
                .map(|(factory,transform)|factory.spawn(transform.clone()))
                .collect();
            //build bullets
            for config in parts{
                entities
                    .build_entity()
                    .with(config.sprite,&mut sprite_render)
                    .with(config.location,&mut transforms)
                    .with(config.movement,&mut movements)
                    .with(config.path_data,&mut path_following)
                    .build();
            }
            /*for (factory,transform) in (factories,transforms).join(){
                to_spawn.push()
                    entities
                        .build_entity()
                        .with(factory.sprites[0].clone(),&mut sprite_render)
                        .with(Transform::default(),&mut transforms)
                        .with(Movement{speed:1.,angle:0.},&mut movements)
                        .with(PathFollowing::new(factory.path.clone()),&mut path_following)
                        .build();
            }
                */
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
