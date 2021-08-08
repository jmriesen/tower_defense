use amethyst::{prelude::*, GameData, StateEvent};

use crate::ground::Ground;

use super::utility::set_up_sprites;

pub struct LoadLevel {
    file_name: std::path::PathBuf,
    next: Option<Box<dyn State<GameData<'static, 'static>, StateEvent>>>,
}
impl LoadLevel {
    pub fn new(
        file_name: std::path::PathBuf,
        next: Box<dyn State<GameData<'static, 'static>, StateEvent>>,
    ) -> Self {
        Self {
            next: Some(next),
            file_name,
        }
    }
}

impl SimpleState for LoadLevel {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        set_up_sprites(world);
        let ground = Ground::read(self.file_name.to_str().unwrap());

        ground.create_tile_map(world);
        ground.create_camera(world);
        ground.create_enemy_factories(world);

        world.insert(ground);
    }

    fn fixed_update(&mut self, _data: StateData<'_, GameData>) -> SimpleTrans {
        if self.next.is_some() {
            Trans::Push(self.next.take().unwrap())
        } else {
            Trans::Pop
        }
    }
}
