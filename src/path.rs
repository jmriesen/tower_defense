use amethyst::{
    core::{
        transform::Transform,
    },
    ecs::{Read,Join,Component,DenseVecStorage,WriteStorage,ReadStorage,System,SystemData},
    derive::SystemDesc,
};
use super::movement::Movement;
use super::ground::Ground;

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
        Read<'s, Option<Ground>>,
    );

    fn run(&mut self, (mut pathfollowings,transforms,mut movements,ground): Self::SystemData) {
        if let Some(ground) = &*ground {
            for (mut _path_following,transform,movement) in
                (&mut pathfollowings,&transforms,&mut movements).join() {
                    let pos = transform.translation();
                    if let Some(mov) = ground.direction_at((pos.x,pos.y)){
                        *movement = mov;
                    }
                }
        }
    }
}
