use super::Bullet;
use super::BulletLaunching;
use amethyst::ecs::{Component, DenseVecStorage};
impl Component for BulletLaunching {
    type Storage = DenseVecStorage<Self>;
}
impl Component for Bullet {
    type Storage = DenseVecStorage<Self>;
}
