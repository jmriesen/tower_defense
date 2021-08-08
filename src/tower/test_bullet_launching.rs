use super::*;
use std::f32::consts::PI;
fn compare_angles(actual: Option<Vec<Movement>>, expected: Vec<f32>) {
    for (movement, expected) in actual.unwrap().iter().zip(expected) {
        assert_eq!(movement.angle, expected);
    }
}
#[test]
fn if_shooting_multiple_bullets_they_are_evenly_spaced() {
    let mut tower = BulletLaunching::new(Duration::new(1, 0), 3, PI);
    tower.angle = Some(0.);
    let expected = vec![-PI / 2., 0., PI / 2.];
    compare_angles(tower.calculate_launch_trajectories(), expected);
    let mut tower = BulletLaunching::new(Duration::new(1, 0), 2, PI / 2.);
    tower.angle = Some(0.);
    let expected = vec![-PI / 4., PI / 4.];
    compare_angles(tower.calculate_launch_trajectories(), expected);
}
#[test]
fn the_angle_should_not_have_to_be_zero() {
    let mut tower = BulletLaunching::new(Duration::new(1, 0), 3, PI);
    tower.angle = Some(1.);
    let mut expected = vec![-PI / 2., 0., PI / 2.];
    expected.iter_mut().for_each(|angle| *angle += 1.);
    compare_angles(tower.calculate_launch_trajectories(), expected);

    let mut tower = BulletLaunching::new(Duration::new(1, 0), 2, PI / 2.);
    tower.angle = Some(2.);
    let mut expected = vec![-PI / 4., PI / 4.];
    expected.iter_mut().for_each(|angle| *angle += 2.);
    compare_angles(tower.calculate_launch_trajectories(), expected);
}
