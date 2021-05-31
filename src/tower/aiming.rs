use amethyst::{
    core::{
        transform::Transform,
    },
    ecs::{Join,WriteStorage,ReadStorage,System,SystemData},
    derive::SystemDesc,
};
use super::Tower;
use super::super::enemy::Enemy;
use super::super::ground::TILE_SIZE;



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
            let target = check_range(tower,tower_trans,min);
            tower.angle = calculate_angle(tower,tower_trans,target);
        }
    }
}
fn check_range<'a,'b>(_tower:&Tower,tower_trans: &Transform,target:Option<(&'a Enemy,&'b Transform)>)->Option<(&'a Enemy,&'b Transform)>{
    let (_enemy, enemy_trans) = target?;
    if distance_sqared(tower_trans,enemy_trans) < (5.*TILE_SIZE as f32).powf(2.) {
        target
    }else {
       None
    }
}
fn calculate_angle(_tower:&Tower,tower_trans: &Transform,target:Option<(&Enemy,&Transform)>)->Option<f32>{
    let (_enemy, enemy_trans) = target?;
    let enemy_pos = enemy_trans.translation();
    let tower_pos = tower_trans.translation();
    let angle = (enemy_pos.y-tower_pos.y).atan2(enemy_pos.x-tower_pos.x);
    Some(angle)
}


fn distance_sqared(t1:&Transform, t2:&Transform)->f32{
    let pos1 = t1.translation();
    let pos2 = t2.translation();
    (pos1.x-pos2.x).powf(2.) + (pos1.y-pos2.y).powf(2.) 
}
