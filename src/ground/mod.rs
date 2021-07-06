use serde::{Serialize, Deserialize};
use amethyst::{
    core::{
        ecs::{
            Component,
            DenseVecStorage
        },
    },
};
//There are two coords systems. One witch is the global and one that is the tile.
pub mod tiles;
mod utility;
mod pathfinding;
pub mod unit_conversions;
pub use unit_conversions::*;

pub use tiles::Tile;

//right now any scaling is not going to be done in here.

#[derive(Deserialize, Serialize,)]
pub struct Ground{
    colum:u32,
    rows:u32,
    //map is always a rectangle.
    map:Vec<Vec<Tile>>,
    pub sink_points:Vec<LatticePoint>,
    pub source_points:Vec<LatticePoint>,
}

impl Ground{
    pub fn new(colum:u32,rows:u32)->Self{
        Self{
            colum,
            rows,
            map:vec![vec![Tile::Path;colum as usize];rows as usize],
            sink_points: vec![],
            source_points: vec![],
        }
    }
    pub fn bounds_check(&self,point:LatticePoint)->bool{
        0 <=point.x && point.x<self.colum as isize && 0 <=point.y && point.y<self.rows as isize
    }

    pub fn map(&self,point:LatticePoint)->Option<&Tile>{
        if self.bounds_check(point) {
            Some(&self.map[point.y as usize][point.x as usize])
        }else{
            None
        }
    }

    pub fn map_mut(&mut self,point:LatticePoint)->Option<&mut Tile>{
        if self.bounds_check(point) {
            Some(&mut self.map[point.y as usize][point.x as usize])
        }else{
            None
        }
    }

}
impl Component for Ground {
    type Storage = DenseVecStorage<Self>;
}

impl Default for Ground{
    fn default()->Self{
        Ground::new(10,10)
    }
}
