use amethyst::{
    ecs::{Entities,Join,ReadStorage,System,SystemData,WriteStorage,Write},
    derive::SystemDesc,
};

use super::enemy::{
    Enemy,
    Helth,
};

use super::player::Money;

#[derive(SystemDesc)]
pub struct DeathSystem;

impl<'s> System<'s> for DeathSystem{
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Enemy>,
        ReadStorage<'s, Helth>,
        Write<'s, Money>,
    );

    fn run(&mut self, (entities, enemys, helth, mut money): Self::SystemData) {
        for (entity, helth) in (&entities, &helth).join(){
            if let Helth(0) = helth{
                if let Some(_) = enemys.get(entity){
                    entities.delete(entity);
                    money.amount +=1;
                }
            }
        }

    }
}
