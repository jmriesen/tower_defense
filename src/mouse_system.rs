use amethyst::{
    prelude::*,
    renderer::{Camera,SpriteRender},
    core::{
        transform::Transform,
        bundle::SystemBundle,
        math::{Point3},
    },
    ecs::{
        DispatcherBuilder,
        ReadStorage,
        Join,
        Read,ReadExpect,Entities,WriteStorage,System,SystemData},
    derive::SystemDesc,
    shrev::{EventChannel,ReaderId},
    input::{InputHandler,StringBindings},
    window::ScreenDimensions,
};

use std::time::Duration;
use amethyst::Error;
use super::tower::Tower;
use super::ground::Ground;
use super::sprites_management::SpriteReasorces;

pub struct PlaceTower;

#[derive(SystemDesc)]
#[system_desc(name(PlaceTowerSystemDesc))]
pub struct PlaceTowerSystem{
    #[system_desc(event_channel_reader)]
    reader: ReaderId<PlaceTower>,
}

impl PlaceTowerSystem{
    pub fn new(reader: ReaderId<PlaceTower>) -> Self {
        Self { reader }
    }
}
//This should probably be split into two systems
impl<'s> System<'s> for PlaceTowerSystem{
    type SystemData = (
        Entities<'s>,
        Read<'s, EventChannel<PlaceTower>>,
        WriteStorage<'s, Tower>,
        WriteStorage<'s, Transform>,
        Read<'s, InputHandler<StringBindings>>,
        ReadExpect<'s, SpriteReasorces<Tower>>,
        WriteStorage<'s, SpriteRender>,
        ReadExpect<'s, ScreenDimensions>,
        ReadExpect<'s, Ground>,
        ReadStorage<'s, Camera>,
    );

    fn run(&mut self, (entities, channel, mut enemy_factories,mut transforms, input, tower_sprites, mut sprite_render, dimensions, ground,camra): Self::SystemData) {
/*        for _event in channel.read(&mut self.reader) {
            //There is no real reason this needs to be a system any more
            //possibly more it into state instead.
            let point = {
                //Only supporting one camra at the moment.
                let (camra, transform) = (&camra, &transforms).join().next().unwrap();
                let (x,y) = input.mouse_position().unwrap();
                camra.screen_to_world_point(
                    Point3::new(x, y, 1.0),
                    dimensions.diagonal(),
                    transform
                )
            };
            let mut transform = Transform::default();
            transform.set_translation_xyz(point.x ,point.y,0.);
            entities
                .build_entity()
                .with(transform,&mut transforms)
                .with(Tower::new(Duration::new(1, 0)), &mut enemy_factories)
                .with(tower_sprites.get(0), &mut sprite_render)
                .build();
        }
        */
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
            PlaceTowerSystemDesc::default().build(world),
            "PlaceTowerSystemDesc",
            &[],
        );
        Ok(())
    }
}
