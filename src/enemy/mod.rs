use amethyst::{
    prelude::*,
    renderer::{SpriteRender},
    core::{
        transform::Transform,
        bundle::SystemBundle,
    },
    ecs::{ReadExpect,Join,Component,DenseVecStorage,ReadStorage,DispatcherBuilder,Read,Entities,WriteStorage,System,SystemData},
    derive::SystemDesc,
    Error,
};

use super::sprites_management::SpriteReasorces;
pub mod helth;
pub mod death;
pub mod spawn;

pub use helth::Helth;
pub use death::DeathSystem;
pub use spawn::{EnemyFactory,SpawnEvent};


pub struct Enemy;

impl Component for Enemy{
    type Storage = DenseVecStorage<Self>;
}


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
        builder.add(
            death::DeathSystem.build(world),
            "DeathSystem",
            &[],
        );
 
        Ok(())
    }
}
