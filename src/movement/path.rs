use super::super::ground::{unit_conversions::*, Ground};
use super::Movement;
use amethyst::{
    core::transform::Transform,
    derive::SystemDesc,
    ecs::{
        Component, DenseVecStorage, Join, ReadExpect, ReadStorage, System, SystemData, WriteStorage,
    },
};

pub struct PathFollowing;

impl Component for PathFollowing {
    type Storage = DenseVecStorage<Self>;
}

#[derive(SystemDesc)]
pub struct PathFollowingSystem;

impl<'s> System<'s> for PathFollowingSystem {
    type SystemData = (
        WriteStorage<'s, PathFollowing>,
        ReadStorage<'s, Transform>,
        WriteStorage<'s, Movement>,
        ReadExpect<'s, Ground>,
    );

    fn run(&mut self, (mut pathfollowings, transforms, mut movements, ground): Self::SystemData) {
        let map = ground.get_gradient();
        for (mut _path_following, transform, movement) in
            (&mut pathfollowings, &transforms, &mut movements).join()
        {
            let pos = TilePoint::from(transform.clone());
            let rounded = LatticePoint::from(pos);
            if let Some(grad) = map.get(rounded) {
                let target = rounded + grad;
                let (delta_x, delta_y) = TilePoint::from(target) - pos;
                movement.angle = delta_y.atan2(delta_x)
            }
        }
    }
}
