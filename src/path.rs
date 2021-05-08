use amethyst::{
    core::{
        transform::Transform,
    },
    ecs::{Entities,Join,Component,DenseVecStorage,WriteStorage,ReadStorage,System,SystemData},
    derive::SystemDesc,
};
use std::sync::Arc;
use super::movement::Movement;
use super::ground::Ground;

pub struct Path{
    targets:Vec<Transform>
}
impl Path {
    pub fn new(start:Transform)->Self{
        Path{
            targets:vec![start]
        }
    }

    pub fn add(&mut self,location:Transform){
        self.targets.push(location);
    }
}

pub struct PathFollowing{
    path: Arc<Path>,
    index: usize,
}

impl PathFollowing{
    pub fn new(path:Arc<Path>) -> Self{
        PathFollowing{
            path: path,
            index: 0,
        }
    }
    fn target(&self)->&Transform{
        &self.path.targets[self.index]
    }
    fn at_end(&self)->bool{
        self.path.targets.len() == self.index
    }
}

impl Component for PathFollowing{
    type Storage = DenseVecStorage<Self>;
}

#[derive(SystemDesc)]
pub struct PathFollowingSystem;

impl<'s> System<'s> for PathFollowingSystem{
    type SystemData = (
        WriteStorage<'s, PathFollowing>,
        ReadStorage<'s, Transform>,
        WriteStorage<'s, Movement>,
        ReadStorage<'s, Ground>,
    );

    fn run(&mut self, (mut pathfollowings,transforms,mut movements,ground): Self::SystemData) {
        //let mut to_remove = vec![];
        for ground in (ground).join(){
            for (mut path_following,transform,movement) in
                (&mut pathfollowings,&transforms,&mut movements).join() {
                    let pos = transform.translation();

                    *movement =  ground.direction_at((pos.x,pos.y));
                }
        }
    }
}
