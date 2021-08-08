use amethyst::{
    core::{bundle::SystemBundle, transform::Transform},
    derive::SystemDesc,
    ecs::{
        Component, DenseVecStorage, DispatcherBuilder, Entities, Join, Read, ReadExpect, System,
        SystemData, WriteStorage,
    },
    prelude::*,
    renderer::SpriteRender,
    Error,
};

use super::sprites_management::SpriteReasorces;
pub mod death;
pub mod helth;
pub mod spawn;

pub use death::DeathSystem;
pub use helth::Helth;
pub use spawn::{EnemyFactory, SpawnEvent};

#[derive(Component)]
pub struct Enemy;

#[derive(Debug)]
pub struct EnemyBundle;
impl<'a, 'b> SystemBundle<'a, 'b> for EnemyBundle {
    fn build(
        self,
        world: &mut World,
        builder: &mut DispatcherBuilder<'a, 'b>,
    ) -> Result<(), Error> {
        builder.add(
            spawn::SpawnSystemDesc::default().build(world),
            "spawnSystem",
            &[],
        );
        builder.add(death::DeathSystem.build(world), "DeathSystem", &[]);

        Ok(())
    }
}
