use amethyst::ecs::{Component,DenseVecStorage};
pub struct Helth(usize);
impl Helth {
    pub fn new(hp:usize)->Self{
        Helth(hp)
    }
    pub fn take_damage(&mut self,damage:usize){
        self.0 = self.0.checked_sub(damage).unwrap_or(0);
    }
    pub fn value(&self)->usize{
        self.0
    }
}

impl Component for Helth{
    type Storage = DenseVecStorage<Self>;
}