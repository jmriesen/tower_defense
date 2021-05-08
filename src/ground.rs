use std::collections::VecDeque;
use amethyst::{
    window::ScreenDimensions,
    core::{
        num::FloatConst,
        ecs::{
            Component,
            DenseVecStorage
        }
    }
};
use super::movement::Movement;
// diagonals are causing problems as we might move into an aria that has no valid value.
fn all_directions()->Vec<(isize,isize)>{
    let mut derections = vec![
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
pub struct Ground{
    x_scale:f32,
    y_scale:f32,
    map:Vec<Vec<bool>>,
    sink_points:Vec<(usize,usize)>,
    //cashed value None if invalid.
    gradiant_map : Option<Vec<Vec<(isize,isize)>>>,
}
impl Ground{
    //I should start to handle scaling
    pub fn new(dimensions: &ScreenDimensions,colum:usize,rows:usize)->Self{
        Self{
            x_scale :(colum as f32)/dimensions.width(),
            y_scale :(rows as f32)/dimensions.height(),
            map:vec![vec![true;colum];rows],
            sink_points: vec![],
            gradiant_map:None,
        }
    }

    pub fn map(&self)->&Vec<Vec<bool>>{
        &self.map
    }

    pub fn map_mut(&mut self)->&mut Vec<Vec<bool>>{
        //gradient map has been dirtied.
        self.gradiant_map = None;
        &mut self.map
    }

    pub fn sink_points(&self)->&Vec<(usize,usize)>{
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
    pub fn direction_at(&self,(x,y):(f32,f32))->Movement{

        let gradiant = self.gradiant_map.as_ref().unwrap();
        let (x,y)= gradiant[(y * self.y_scale) as usize][(x *self.x_scale) as usize];
        let speed = if (x,y) == (0,0) {0.} else {1.};

        Movement{
            angle:(y as f32).atan2(x as f32)- f32::FRAC_PI_2(),
            speed
        }
    }
}

impl Component for Ground {
    type Storage = DenseVecStorage<Self>;
}
