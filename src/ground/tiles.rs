use serde::{Deserialize, Serialize};
#[derive(Deserialize, Serialize, Clone, Copy, PartialEq)]
pub enum Tile {
    Path,
    Grass,
    Water,
}

pub use render::TileRenderer;
mod render {
    use super::super::Ground;
    use amethyst::{core::math::Point3, prelude::World, tiles::Tile};
    #[derive(Clone, Default)]
    pub struct TileRenderer;
    impl Tile for TileRenderer {
        fn sprite(&self, coords: Point3<u32>, world: &World) -> Option<usize> {
            let ground = world.fetch::<Ground>();
            //default I want positive y as up, default is down.
            let pos = (coords.x as isize, (ground.colum - 1 - coords.y) as isize);
            Some(*ground.map(pos.into())? as usize)
        }
    }
}
