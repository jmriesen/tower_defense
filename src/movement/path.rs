use amethyst::{
    core::{
        transform::Transform,
    },
    ecs::{ReadExpect,Join,Component,DenseVecStorage,WriteStorage,ReadStorage,System,SystemData},
    derive::SystemDesc,
};
use super::Movement;
use super::super::ground::Ground;

pub struct PathFollowing;


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
        ReadExpect<'s, Ground>,
    );

    fn run(&mut self, (mut pathfollowings,transforms,mut movements,ground): Self::SystemData) {
        for (mut _path_following,transform,movement) in
            (&mut pathfollowings,&transforms,&mut movements).join() {
                if let Some(mov) = ground.direction_at(transform){
                    *movement = mov;
                }
            }
    }
}
