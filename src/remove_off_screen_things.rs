use amethyst::{
    ecs::{Entities,Join,ReadStorage,System,SystemData,ReadExpect},
    derive::SystemDesc,
    core::transform::Transform,
};


use super::ground::Ground;
#[derive(SystemDesc)]
pub struct Destry;

impl<'s> System<'s> for Destry{
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Transform>,
        ReadExpect<'s,Ground>,
    );

    fn run(&mut self, (entities, transfroms, ground): Self::SystemData) {
        for (entity, transfrom) in (&entities, &transfroms).join(){
            if !ground.validate_location(&transfrom){
                let _ = entities.delete(entity);
            }
        }

    }
}
