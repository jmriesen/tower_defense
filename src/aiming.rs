use amethyst::{
    core::{
        transform::Transform,
        num::FloatConst,
    },
    ecs::{Entities,Join,WriteStorage,ReadStorage,System,SystemData},
    derive::SystemDesc,
};
use super::tower::Tower;
use super::enemy::Enemy;



#[derive(SystemDesc)]
pub struct AimingSystem;

impl<'s> System<'s> for AimingSystem{
    type SystemData = (
        WriteStorage<'s, Tower>,
        ReadStorage<'s, Enemy>,
        ReadStorage<'s, Transform>,
    );

    fn run(&mut self, (mut towers, enemys, transfroms): Self::SystemData) {
        for (mut tower, tower_trans) in (&mut towers, &transfroms).join(){
            //let mut min = None;
            let min = (&enemys, &transfroms)
                .join()
                .min_by_key(
                    |(_enemy, enemy_trans)|
                    distance_sqared(tower_trans,enemy_trans).round() as isize
                );
            if let Some((_enemy, enemy_trans)) = min{
                let enemy_pos = enemy_trans.translation();
                let tower_pos = tower_trans.translation();
                let angle = (enemy_pos.y-tower_pos.y).atan2(enemy_pos.x-tower_pos.x)- f32::FRAC_PI_2();
                tower.angle = Some(angle);
            }else{
                tower.angle = None;
            }
        }
    }
}

fn distance_sqared(t1:&Transform, t2:&Transform)->f32{
    let pos1 = t1.translation();
    let pos2 = t2.translation();
    (pos1.x-pos2.x).powf(2.) + (pos1.y-pos2.y).powf(2.) 
}
