use serde::{Serialize, Deserialize};
use std::collections::VecDeque;
use amethyst::{
    core::{
        Transform,
        ecs::{
            Component,
            DenseVecStorage
        },
    },
};
//There are two coords systems. One witch is the global and one that is the tile.
pub mod tiles;
mod utility;
mod accessors;
pub const TILE_SIZE:u32 = 64;
use super::movement::Movement;
fn all_directions()->Vec<(isize,isize)>{
    vec![
        (0,1),
        (-1,0),
        (0,-1),
        (1,0),
    ]
}
//right now any scaling is not going to be done in here.

#[derive(Deserialize, Serialize,)]
pub struct Ground{
    colum:u32,
    rows:u32,
    //map is always a rectangle.
    map:Vec<Vec<bool>>,
    sink_points:Vec<(usize,usize)>,
    //cashed value None if invalid.
    #[serde(skip)]
    gradiant_map : Option<Vec<Vec<(isize,isize)>>>,
}
impl Ground{
    pub fn new(colum:u32,rows:u32)->Self{
        Self{
            colum,
            rows,
            map:vec![vec![true;colum as usize];rows as usize],
            sink_points: vec![],
            gradiant_map:None,
        }
    }


    pub fn refresh(&mut self){
        self.gradiant_map = Some(vec![vec![(0,0);self.colum as usize];self.rows as usize]);

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
        let next_pos = (x as isize+x_delta,(y as isize+y_delta));
        if Some(&true) == self.map(next_pos){
            let node = self.gradiant_mut(next_pos).unwrap();
            if *node == (0,0){
                *node = (-x_delta,-y_delta);
                queue.push_back(to_usize(next_pos));
            }
        }
    }


    pub fn direction_at(&self,location:&Transform)->Option<Movement>{
        let (x,y) = Ground::trans_to_tile(location);
        let (grad_x,grad_y) = *self.gradiant((x.round() as isize,y.round() as isize))?;
        let (grad_x,grad_y) = (grad_x as f32,grad_y as f32);
        let (target_x,target_y) = (x.round()+grad_x,y.round()+grad_y);
        let (delta_x,delta_y) = (target_x-x,target_y - y);

        let speed = if delta_x.abs()<0.05&&delta_y.abs()<0.05 {0.} else {1.};
        Some(Movement{
            angle:delta_y.atan2(delta_x),
            speed
        })
    }

}
fn to_usize((x,y):(isize,isize))->(usize,usize){
    (x as usize,y as usize)
}

impl Component for Ground {
    type Storage = DenseVecStorage<Self>;
}

