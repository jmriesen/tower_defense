use amethyst::{
    input::{is_close_requested, is_key_down, VirtualKeyCode, InputEvent},
    prelude::*,
    shrev::{EventChannel},
    GameData,
    StateEvent,
    core::Transform,
};

use crate::enemy::EnemyFactory;
use crate::ground::{
    Ground,
    Tile,
    unit_conversions::*,
};
use crate::player::Player;


use super::utility::{
    set_up_sprites,
};

pub struct LoadLevel{
    next:Option<Box<dyn State<GameData<'static,'static>,StateEvent>>>
}
impl LoadLevel{
    pub fn new(next:Box<dyn State<GameData<'static,'static>,StateEvent>>)->Self{
        Self{next:Some(next)}
    }
}

impl SimpleState for LoadLevel{
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        set_up_sprites(world);
//let ground = Ground::load(application_root_dir().unwrap().join("ground.ron")).unwrap();
        world.insert(Player::default());
        let mut ground = Ground::new(10,10);
        for i in 0..9{
         *ground.map_mut((i,1).into()).unwrap() = Tile::Grass;
         *ground.map_mut((9-i,3).into()).unwrap() = Tile::Grass;
    }

        ground.sink_points.push((0,0).into());
        ground.create_tile_map(world);
        ground.create_camera(world);

        ground.write("ground").unwrap();

        world.insert(ground);

        world.create_entity()
            .with(Transform::from(TilePoint{x:4.,y:5.}))
            .with(EnemyFactory)
            .build();
    }


    fn fixed_update(&mut self, _data: StateData<'_, GameData>) -> SimpleTrans{
        if self.next.is_some(){
            Trans::Push(self.next.take().unwrap())
        }else{
            Trans::Pop
        }
    }
}
