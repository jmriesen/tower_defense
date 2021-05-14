use serde::{Serialize, Deserialize};
use std::collections::VecDeque;
use amethyst::{
    prelude::*,
    window::ScreenDimensions,
    core::{
        num::FloatConst,
        ecs::{
            Component,
            DenseVecStorage
        },
        math::{Vector3,Point3,},
        Transform,
    },
    tiles::{Tile,TileMap},
    prelude::World,
    renderer::Camera,
};
//There are two coords systems. One witch is the global and one that is the tile.
const TILE_SIZE:u32 = 64;
use super::movement::Movement;
// diagonals are causing problems as we might move into an aria that has no valid value.
//We get in the tiles but it dose not really look like that think about this.
fn all_directions()->Vec<(isize,isize)>{
    let derections = vec![
//        (1,1),
        (0,1),
 //       (-1,1),
        (-1,0),
//        (-1,-1),
        (0,-1),
//        (1,-1),
        (1,0),
    ];
    /*for x in -1..=1 {
    for y in -1..=1 {
    derections.push((-x,-y));
}
}
     */
    derections
}
//right now any scaling is not going to be done in here.

#[derive(Deserialize, Serialize,)]
pub struct Ground{
    colum:u32,
    rows:u32,
    //TODO think about making invariant between column and rows by using arrays possibly.
    map:Vec<Vec<bool>>,
    sink_points:Vec<(usize,usize)>,
    //cashed value None if invalid.
    gradiant_map : Option<Vec<Vec<(isize,isize)>>>,
}
impl Ground{
    pub fn new(_dimensions: &ScreenDimensions,colum:u32,rows:u32)->Self{
        Self{
            colum,
            rows,
            map:vec![vec![true;colum as usize];rows as usize],
            sink_points: vec![],
            gradiant_map:None,
        }
    }

    pub fn _map(&self)->&Vec<Vec<bool>>{
        &self.map
    }

    pub fn map_mut(&mut self)->&mut Vec<Vec<bool>>{
        //gradient map has been dirtied.
        self.gradiant_map = None;
        &mut self.map
    }

    pub fn _sink_points(&self)->&Vec<(usize,usize)>{
        &self.sink_points
    }

    pub fn sink_points_mut(&mut self)->&mut Vec<(usize,usize)>{
        //gradient map has been dirtied.
        self.gradiant_map = None;
        &mut self.sink_points
    }

    pub fn refresh(&mut self){
        let mut gradiant_map = vec![];
        for row in &self.map{
            gradiant_map.push(vec![(0,0);row.len()])
        }
        self.gradiant_map = Some(gradiant_map);

        let mut queue = VecDeque::from(self.sink_points.clone());
        while let Some(point) = queue.pop_front(){
            for derection in all_directions() {
                self.mark(point,derection,&mut queue);
            }
        }
        for (x,y) in &self.sink_points{
            self.gradiant_map.as_mut().unwrap()[*y][*x] = (0,0);
        }
    }
    fn mark(&mut self,(x,y):(usize,usize),(x_delta,y_delta):(isize,isize),queue:&mut VecDeque<(usize,usize)>){
        use std::convert::TryFrom;
        if let (Ok(x),Ok(y)) = (usize::try_from(x as isize+x_delta),usize::try_from(y as isize+y_delta)){
            if let Some(row) = self.map.get(y){
                if *row.get(x).unwrap_or(&false){
                    if self.gradiant_map.as_mut().unwrap()[y][x] == (0,0){
                        self.gradiant_map.as_mut().unwrap()[y][x] = (-x_delta,-y_delta);
                        queue.push_back((x,y));
                    }
                }
            }

        }
    }
    fn _tile_to_trans((x,y):(f32,f32))->(f32,f32){
        (x * TILE_SIZE as f32,y * TILE_SIZE  as f32)
    }
    fn trans_to_tile((x,y):(f32,f32))->(f32,f32){
        (x /TILE_SIZE as f32,y /TILE_SIZE as f32)
    }
    pub fn direction_at(&self,(x,y):(f32,f32))->Option<Movement>{
        let gradiant = self.gradiant_map.as_ref().unwrap();
        let (x,y) = Ground::trans_to_tile((x,y));
        let (grad_x,grad_y) = gradiant[y.round() as usize][x.round() as usize];
        let (target_x,target_y) = (x.round()+grad_x as f32,y.round()+grad_y as f32);
        let (delta_x,delta_y) = (target_x-x,target_y - y);

        let speed = if delta_x.abs()<0.05&&delta_y.abs()<0.05 {0.} else {1.};
        Some(Movement{
            angle:delta_y.atan2(delta_x)- f32::FRAC_PI_2(),
            speed
        })
    }
    pub fn create_tile_map(&self,world:&mut World){
        let map = TileMap::<SimpleTile>::new(
            Vector3::new(self.colum, self.rows, 1), // The dimensions of the map
            Vector3::new(TILE_SIZE,TILE_SIZE, 1), // The dimensions of each tile
            Some(super::state::load_sheet(world, "logo")),
        );

        let (width,hight) = ((TILE_SIZE*self.colum) as f32,(TILE_SIZE*self.rows) as f32);

        let offset = -(TILE_SIZE as f32 );
        let mut transform = Transform::default();
        transform.append_translation_xyz(0.,offset,0.);
        transform.append_translation_xyz(width/2.,hight/2.,0.);
        world.create_entity()
            .with(map)
            .with(transform)
            .build();
    }
    pub fn create_camera(&self,world: &mut World) {
        let (width,hight) = ((TILE_SIZE*self.colum) as f32,(TILE_SIZE*self.rows) as f32);
        let mut transform = Transform::default();
        let offset = -(TILE_SIZE as f32 /2.);
        transform.append_translation_xyz(0.,0.,1.);
        transform.append_translation_xyz(offset,offset,0.);
        transform.append_translation_xyz(width/2.,hight/2.,0.);

        world
            .create_entity()
            .with(Camera::standard_2d(width,hight))
            .with(transform)
            .build();
}
}

impl Component for Ground {
    type Storage = DenseVecStorage<Self>;
}

#[derive(Clone, Default)]
pub struct SimpleTile;
impl Tile for SimpleTile {
    fn sprite(&self, coords: Point3<u32>, world: &World) -> Option<usize> {
        if let Some(ground) = (&*world.fetch::<Option<Ground>>()).as_ref(){
            //default I want positive y as up, default is down.
            let (x,y) = (coords.x as usize,(ground.colum-1-coords.y) as usize);
            let index = if ground.map[y][x] {0} else {1};
            Some(index)
        }else{
            None
        }
    }
}
