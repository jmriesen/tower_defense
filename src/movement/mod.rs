use amethyst::{
    core::transform::Transform,
    derive::SystemDesc,
    ecs::{Component, DenseVecStorage, Join, ReadStorage, System, SystemData, WriteStorage},
};

pub mod path;
#[derive(Component)]
pub struct Movement {
    pub angle: f32,
    pub speed: f32,
}

#[derive(SystemDesc)]
pub struct MovementSystem;

impl<'s> System<'s> for MovementSystem {
    type SystemData = (ReadStorage<'s, Movement>, WriteStorage<'s, Transform>);

    fn run(&mut self, (movements, mut transfoms): Self::SystemData) {
        for (movement, transform) in (&movements, &mut transfoms).join() {
            transform.set_rotation_2d(movement.angle);
            transform.move_right(movement.speed);
        }
    }
}
