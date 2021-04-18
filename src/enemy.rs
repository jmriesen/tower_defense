use std::sync::Arc;
use amethyst::{
    prelude::*,
    renderer::{SpriteRender, SpriteSheet},
    core::{
        transform::Transform,
        math::Vector3,
        num::real::Real,
        num::FloatConst,
        bundle::SystemBundle,
    },
    ecs::{DispatcherBuilder,Read,Entities,Join,Component,DenseVecStorage,WriteStorage,System,SystemData,ReadStorage},
    derive::SystemDesc,
    shrev::{EventChannel,ReaderId},
};
use amethyst::Error;


pub struct EnemyFactory{
    sprites: Vec<SpriteRender>,
    path : Arc<super::path::Path>,

}
impl EnemyFactory{
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
}
pub struct SpawnEvent;
use super::path::PathFollowing;

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
        Read<'s, Option<EnemyFactory>>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, PathFollowing>,
        WriteStorage<'s, SpriteRender>,
    );

    fn run(&mut self, (entities, channel, factory, mut transforms, mut path_following, mut sprite_render): Self::SystemData) {
        for _event in channel.read(&mut self.reader) {
            if let Some(factory) = factory.as_ref(){

                entities
                    .build_entity()
                    .with(factory.sprites[0].clone(),&mut sprite_render)
                    .with(Transform::default(),&mut transforms)
                    .with(PathFollowing::new(factory.path.clone()),&mut path_following)
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
