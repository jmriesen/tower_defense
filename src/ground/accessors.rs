use super::*;
impl Ground{
    pub fn sink_points_mut(&mut self)->&mut Vec<(usize,usize)>{
        //gradient map has been dirtied.
        self.gradiant_map = None;
        &mut self.sink_points
    }
    fn bounds_check(&self,(x,y): (isize,isize))->bool{
        0 <=x && x<self.colum as isize && 0 <=y && y<self.rows as isize
    }

    pub fn map(&self,(x,y):(isize,isize))->Option<&bool>{
        if self.bounds_check((x,y)) {
            Some(&self.map[y as usize][x as usize])
        }else{
            None
        }
    }

    pub fn map_mut(&mut self,(x,y):(isize,isize))->Option<&mut bool>{
        self.gradiant_map = None;
        if self.bounds_check((x,y)) {
            Some(&mut self.map[y as usize][x as usize])
        }else{
            None
        }
    }

    pub fn gradiant(&self,(x,y):(isize,isize))->Option<&(isize,isize)>{
        if self.bounds_check((x,y)) {
            Some(&self.gradiant_map.as_ref()?[y as usize][x as usize])
        }else{
            None
        }
    }

    pub fn gradiant_mut(&mut self,(x,y):(isize,isize))->Option<&mut (isize,isize)>{
        if self.bounds_check((x,y)) {
            Some(&mut self.gradiant_map.as_mut()?[y as usize][x as usize])
        }else{
            None
        }
    }
}
