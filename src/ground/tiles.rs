use amethyst::{
    core::{
        math::{Point3},
    },
    tiles::{Tile},
    prelude::World,
};
use super::Ground;
#[derive(Clone, Default)]
pub struct GroundTile;
impl Tile for GroundTile{
    fn sprite(&self, coords: Point3<u32>, world: &World) -> Option<usize> {
            let ground = world.fetch::<Ground>();
            //default I want positive y as up, default is down.
            let pos = (coords.x as isize,(ground.colum-1-coords.y) as isize);
            let index = if *ground.map(pos)? {0} else {1};
            Some(index)
    }
}
