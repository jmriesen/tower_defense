use amethyst::{
    core::{
        transform::Transform,
    },
    ecs::{ReadExpect,Join,Component,DenseVecStorage,WriteStorage,ReadStorage,System,SystemData},
    derive::SystemDesc,
};
use super::Movement;
use super::super::ground::{
    Ground,
    unit_conversions::*,
};

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
        let map = ground.get_gradient();
        for (mut _path_following,transform,movement) in
            (&mut pathfollowings,&transforms,&mut movements).join() {
                //TODO this code needs to be refactored.
                let pos = TilePoint::from(transform.clone());
                let rounded = LatticePoint::from(pos);
                if let Some(grad) = map[rounded.y as usize][rounded.x as usize]{
                    let target = rounded + grad;
                    let (delta_x,delta_y) = TilePoint::from(target) - pos;
                        //(target_x as f32 -pos.0,target_y as f32 - pos.1);
                    movement.angle =delta_y.atan2(delta_x)
                }
            }
    }
}
