use amethyst::{
    core::{
        transform::Transform,
    },
    ecs::{Join,Component,DenseVecStorage,WriteStorage,ReadStorage,System,SystemData},
    derive::SystemDesc,
};
pub struct Movement {
    pub angle : f32,
    pub speed : f32,
}


impl Component for Movement{
    type Storage = DenseVecStorage<Self>;
}

#[derive(SystemDesc)]
pub struct MovementSystem;

impl<'s> System<'s> for MovementSystem{
    type SystemData = (
        ReadStorage<'s, Movement>,
        WriteStorage<'s, Transform>,
    );

    fn run(&mut self, (movements,mut transfoms): Self::SystemData) {
        for (movement,transform) in (&movements,&mut transfoms).join() {
            transform.set_rotation_2d(movement.angle);
            transform.move_up(movement.speed);
        }
    }
}
