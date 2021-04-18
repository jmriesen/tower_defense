use amethyst::{
    core::{
        transform::Transform,
        math::Vector3,
        num::real::Real,
        num::FloatConst,
    },
    ecs::{Entities,Join,Component,DenseVecStorage,WriteStorage,System,SystemData,ReadStorage},
    derive::SystemDesc,
};
use std::sync::Arc;

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
        Entities<'s>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, PathFollowing>,
    );

    fn run(&mut self, (entities,mut transforms,mut pathfollowings): Self::SystemData) {
        let mut to_remove = vec![];

        for (entity,transform, mut pathFollowing) in (&entities,&mut transforms, &mut pathfollowings).join() {
            if pathFollowing.at_end(){
                to_remove.push(entity)
            }else {
                let target = pathFollowing.target();

                let distance_to_target = (target.translation() - transform.translation()).norm();
                if distance_to_target < 10.{
                    pathFollowing.index +=1;
                }else{

                    let angle = get_angle_towards(transform,target);
                    transform.set_rotation_2d(angle);
                    transform.move_up(1.);
                }
            }

        }
        for entity in to_remove{
            pathfollowings.remove(entity);
        }
    }

}
fn get_angle_towards(start: &Transform,target:&Transform)->f32{
    let start_pos = start.translation();
    let target_pos = target.translation();

    (target_pos.y-start_pos.y).atan2(target_pos.x-start_pos.x) - f32::FRAC_PI_2()
}
