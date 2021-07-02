use std::collections::VecDeque;
use std::convert::TryFrom;
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
pub struct GradientMap {
    map:Vec<Vec<Option<(isize,isize)>>>
}
impl GradientMap {
    fn new(hight:usize,width:usize) ->Self{
        GradientMap{
            map:vec![vec![None;width];hight]
        }
    }
    pub fn get(&self,point:LatticePoint)->Option<(isize,isize)>{
        let x = usize::try_from(point.x).ok()?;
        let y = usize::try_from(point.x).ok()?;
        *self.map.get(point.y as usize)?.get(point.x as usize)?

    }
    pub fn set(&mut self,point:LatticePoint,data:Option<(isize,isize)>){
        if !point.x.is_negative() && !point.y.is_negative(){
            if let Some(row) = self.map.get_mut(point.y as usize){
                if let Some(entry) = row.get_mut(point.x as usize){
                    *entry = data;
                }
            }
        }
    }
}
impl Ground{
    pub fn get_gradient(&self)->GradientMap{
        let mut map= GradientMap::new(self.rows as usize,self.colum as usize);

        let mut queue:VecDeque<LatticePoint> = self.sink_points.clone().into();

        while let Some(point) = queue.pop_front(){
            for derection in all_directions() {
                let point_to_mark = point + derection;
                if let Some(&Tile::Path) = self.map(point_to_mark){
                    if  None == map.get(point_to_mark){
                        map.set(point_to_mark, Some((-derection.0,-derection.1)));
                        queue.push_back(point_to_mark);
                    }
                }
            }
        }

        for pos in &self.sink_points{
            map.set(*pos, None);
        }
        map
    }
}
