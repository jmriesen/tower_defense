use amethyst::{
    ecs::{Component,DenseVecStorage},
};
use super::Tower;
use super::Bullet;
impl Component for Tower{
    type Storage = DenseVecStorage<Self>;
}
impl Component for Bullet{
    type Storage = DenseVecStorage<Self>;
}
