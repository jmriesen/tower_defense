use std::collections::VecDeque;
use super::Ground;
use super::tiles::Tile;
use super::unit_conversions::*;
fn all_directions()->Vec<(isize,isize)>{
    vec![
        (0,1),
        //(-1,1),
        (-1,0),
        //(-1,-1),
        (0,-1),
        //(1,-1),
        (1,0),
        //(1,1),
    ]
}
pub type GradientMap = Vec<Vec<Option<(isize,isize)>>>;
impl Ground{
    pub fn get_gradient(&self)->GradientMap{
        let mut map:GradientMap = vec![vec![None;self.colum as usize];self.rows as usize];

        //let mut queue = VecDeque::from(self.sink_points.clone());
        let mut queue:VecDeque<LatticePoint> = self.sink_points.clone().into();

        while let Some(point) = queue.pop_front(){
            for derection in all_directions() {
                let point_to_mark = point + derection;
                if self.bounds_check(point_to_mark){
                    let (x,y) = (point_to_mark.x as usize,point_to_mark.y as usize);
                    if self.map[y][x] == Tile::Path && None == map[y][x]{
                        map[y][x] = Some((-derection.0,-derection.1));
                        queue.push_back(point_to_mark);
                    }
                }
            }
        }
        map
    }
}
