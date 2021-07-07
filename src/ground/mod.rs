use serde::{Serialize, Deserialize};
use amethyst::{
    core::{
        ecs::{
            Component,
            DenseVecStorage
        },
    },
    prelude::Config,
    config
};
//There are two coords systems. One witch is the global and one that is the tile.
pub mod tiles;
mod utility;
mod pathfinding;
pub mod unit_conversions;
pub use unit_conversions::*;

pub use tiles::Tile;
pub use std::path::PathBuf;

const LEVEL_FOLDER : &str = "levels";
//right now any scaling is not going to be done in here.

#[derive(Deserialize, Serialize,)]
pub struct Ground{
    #[serde(skip)] 
    name:String,
    colum:u32,
    rows:u32,
    //map is always a rectangle.
    map:Vec<Vec<Tile>>,
    pub sink_points:Vec<LatticePoint>,
    pub source_points:Vec<LatticePoint>,
}

impl Ground{
    pub fn new(name:String,colum:u32,rows:u32)->Self{
        Self{
            name,
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
    pub fn save(&self){
        let mut level = PathBuf::from(LEVEL_FOLDER);
        level.push(self.name.clone());
        level.set_extension("ron");
        self.write(level).unwrap();
    }
    pub fn read(name:&str)->Self{
        let mut level = PathBuf::from(LEVEL_FOLDER);
        level.push(name);
        level.set_extension("ron");
        let mut ground = match Ground::load(level.to_str().unwrap()){
            Ok(ground) => ground,
            Err(config::ConfigError::File(os))=>{
                if os.kind() == std::io::ErrorKind::NotFound{
                    Ground::default()
                }
                else{
                    panic!("{:?},",os)
                }
            }
            Err(other) => panic!("{:?}",other),
        };
        ground.name = name.to_string();
        ground
    }
}
impl Component for Ground {
    type Storage = DenseVecStorage<Self>;
}

impl Default for Ground{
    fn default()->Self{
        Ground::new(String::from(""),10,10)
    }
}
