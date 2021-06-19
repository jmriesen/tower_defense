use amethyst::{
    core::{
        transform::Transform,
    },
    ecs::{Entities,Join,ReadStorage,System,SystemData,WriteStorage,Write},
    derive::SystemDesc,
};
use super::tower::Bullet;
use super::enemy::{
    Enemy,
    Helth,
};
use super::player::Money;



#[derive(SystemDesc)]
pub struct CollitionSystem;

impl<'s> System<'s> for CollitionSystem{
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Transform>,
        ReadStorage<'s, Bullet>,
        ReadStorage<'s, Enemy>,
        WriteStorage<'s, Helth>,
        Write<'s, Money>,
    );

    fn run(&mut self, (entities, transfroms, bullets, enemys, mut helth, mut money): Self::SystemData) {
        let mut to_delete = vec![];
        for (bullet, bullet_trans, _) in (&entities, &transfroms, &bullets).join(){
            for (enemy, enemy_trans, _, mut helth) in (&entities, &transfroms, &enemys, &mut helth).join(){
                if distance_less_then(enemy_trans,bullet_trans, 32.){
                    to_delete.push(bullet.clone());
                    let Helth(ph) = &mut helth;
                    *ph = ph.checked_sub(1).unwrap_or(0);
                    if *ph == 0{
                        to_delete.push(enemy.clone());
                        money.0 +=1;
                        println!("Money {}",money.0);
                    }
                }

            }
        }
        for entity in to_delete{
            let _ = entities.delete(entity);
        }
    }
}

fn distance_less_then(t1:&Transform, t2:&Transform,tolarence:f32)->bool{
    let pos1 = t1.translation();
    let pos2 = t2.translation();
    (pos1.x-pos2.x).powf(2.) + (pos1.y-pos2.y).powf(2.) <tolarence.powf(2.)
}
