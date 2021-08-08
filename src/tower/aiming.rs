use super::super::enemy::Enemy;
use super::super::ground::TILE_SIZE;
use super::BulletLaunching;
use amethyst::{
    core::transform::Transform,
    derive::SystemDesc,
    ecs::{Join, ReadStorage, System, SystemData, WriteStorage},
};

#[derive(SystemDesc)]
pub struct AimingSystem;

impl<'s> System<'s> for AimingSystem {
    type SystemData = (
        WriteStorage<'s, BulletLaunching>,
        ReadStorage<'s, Enemy>,
        WriteStorage<'s, Transform>,
    );

    fn run(&mut self, (mut towers, enemys, mut transfroms): Self::SystemData) {
        for (mut tower, tower_trans) in (&mut towers, &transfroms).join() {
            let min = (&enemys, &transfroms)
                .join()
                .min_by_key(|(_enemy, enemy_trans)| {
                    distance_sqared(tower_trans, enemy_trans).round() as isize
                });
            let target = check_range(tower, tower_trans, min);
            tower.angle = calculate_angle(tower, tower_trans, target);
        }

        for (tower, tower_trans) in (&towers, &mut transfroms).join() {
            // Not sure if this should go here or else where
            // tower_trans.set_rotation(tower.angle);
            if let Some(angle) = tower.angle {
                tower_trans.set_rotation_z_axis(angle - 1.5708);
            }
        }
    }
}

fn check_range<'a, 'b>(
    _tower: &BulletLaunching,
    tower_trans: &Transform,
    target: Option<(&'a Enemy, &'b Transform)>,
) -> Option<(&'a Enemy, &'b Transform)> {
    let (_enemy, enemy_trans) = target?;
    if distance_sqared(tower_trans, enemy_trans) < (2. * TILE_SIZE as f32).powf(2.) {
        target
    } else {
        None
    }
}
fn calculate_angle(
    _tower: &BulletLaunching,
    tower_trans: &Transform,
    target: Option<(&Enemy, &Transform)>,
) -> Option<f32> {
    let (_enemy, enemy_trans) = target?;
    let enemy_pos = enemy_trans.translation();
    let tower_pos = tower_trans.translation();
    let angle = (enemy_pos.y - tower_pos.y).atan2(enemy_pos.x - tower_pos.x);
    Some(angle)
}

fn distance_sqared(t1: &Transform, t2: &Transform) -> f32 {
    let pos1 = t1.translation();
    let pos2 = t2.translation();
    (pos1.x - pos2.x).powf(2.) + (pos1.y - pos2.y).powf(2.)
}
