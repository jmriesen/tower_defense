use amethyst::core::Transform;
use std::ops::{Add,Sub};

pub const TILE_SIZE:u32 = 64;
use serde::{Serialize, Deserialize};
#[derive(Deserialize, Serialize, Clone, Copy, PartialEq)]
pub struct TilePoint{
    pub  x: f32,
    pub y: f32,
}

#[derive(Deserialize, Serialize, Clone, Copy, PartialEq)]
pub struct LatticePoint{
    pub x: isize,
    pub y: isize,
}

impl From<(isize,isize)> for LatticePoint{
    fn from((x,y):(isize,isize))->Self{
        LatticePoint{x,y}
    }
}
impl From<LatticePoint> for TilePoint{
    fn from(point:LatticePoint)->Self{
        TilePoint{
            x:point.x as f32,
            y:point.y as f32,
        }
    }
}
impl From<TilePoint> for LatticePoint{
    fn from(point:TilePoint)->Self{
        LatticePoint{
            x:point.x.round() as isize,
            y:point.y.round() as isize,
        }
    }
}
impl From<Transform> for LatticePoint{
    fn from(transforms:Transform)->Self{
        LatticePoint::from(
            TilePoint::from(
                transforms
            )
        )
    }
}

impl From<(f32,f32)> for TilePoint{
    fn from((x,y):(f32,f32))->Self{
        TilePoint{x,y}
    }
}

impl From<Transform> for TilePoint{
    fn from(location:Transform)->Self{
        let pos = location.translation();
        TilePoint{
            x:pos.x/TILE_SIZE as f32,
            y:pos.y/TILE_SIZE as f32,
        }
    }
}
impl From<TilePoint> for Transform{
    fn from(point:TilePoint) -> Self{
        let mut transform = Transform::default();
        transform.set_translation_xyz(
            point.x * TILE_SIZE  as f32,
            point.y * TILE_SIZE  as f32,
            0.);
        transform
    }
}
impl Add<(isize,isize)> for LatticePoint{
    type Output = Self;
    fn add(self, (x,y): (isize,isize)) -> Self {
        Self {
            x: self.x + x,
            y: self.y + y,
        }
    }
}

impl Sub for TilePoint {
    type Output = (f32,f32);

    fn sub(self, other: Self) -> Self::Output {
            (self.x - other.x, self.y - other.y)
    }
}
