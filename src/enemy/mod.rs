use amethyst::{
    prelude::*,
    renderer::{SpriteRender},
    core::{
        transform::Transform,
        bundle::SystemBundle,
    },
    ecs::{ReadExpect,Join,Component,DenseVecStorage,ReadStorage,DispatcherBuilder,Read,Entities,WriteStorage,System,SystemData},
    derive::SystemDesc,
};

use super::sprites_management::SpriteReasorces;
pub mod helth;
pub mod death;
pub mod spawn;

pub use helth::Helth;
pub use death::DeathSystem;
pub use spawn::{EnemyFactory,MyBundle,SpawnEvent};


pub struct Enemy;

impl Component for Enemy{
    type Storage = DenseVecStorage<Self>;
}
