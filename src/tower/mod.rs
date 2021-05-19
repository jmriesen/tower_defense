mod components;
pub mod fireing_system;
pub mod aiming;
use std::time::Duration;

pub enum TurretState{
    Ready,
    CoolingDown(Duration),
}

pub struct Tower{
    pub reload_time:Duration,
    pub state:TurretState,
    pub angle:Option<f32>,
}

impl Tower{
    pub fn new(reload_time:Duration)->Self{
        Tower{
            reload_time,
            state:TurretState::Ready,
            angle : None,
        }
    }
}



pub struct Bullet;

