use amethyst::{
    ecs::{Component,DenseVecStorage},
};
use super::BulletLaunching;
use super::Bullet;
impl Component for BulletLaunching{
    type Storage = DenseVecStorage<Self>;
}
impl Component for Bullet{
    type Storage = DenseVecStorage<Self>;
}
