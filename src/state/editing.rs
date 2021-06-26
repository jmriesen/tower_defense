use amethyst::{
    input::{is_close_requested, is_key_down, VirtualKeyCode, InputEvent},
    prelude::*,
    shrev::{EventChannel},
};

use crate::enemy::SpawnEvent;
use crate::ground::{Ground};
use crate::player::set_up_money;


use super::utility::{
    get_mouse_position,
    set_up_sprites,
};

#[derive(Default)]
pub struct Editing;

impl SimpleState for Editing{
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        set_up_sprites(world);

        set_up_money(world);
        let mut ground = Ground::new(10,10);
        for i in 0..9{
         *ground.map_mut((i,1)).unwrap() = false;
         *ground.map_mut((9-i,3)).unwrap() = false;
    }

        ground.sink_points_mut().push((0,0));
        ground.refresh();
        ground.create_tile_map(world);
        ground.create_camera(world);

        ground.write("ground").unwrap();

        world.insert(ground);
    }

    fn handle_event(
        &mut self,
        data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        match event{
            StateEvent::Window(event) =>  {
                // Check if the window should be closed
                if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
                    Trans::Quit
                }else{
                    Trans::None
                }

            }
            StateEvent::Input(InputEvent::ActionReleased(action)) => {
                match  action.as_str() {
                    "shoot" => {
                        let world = data.world;
                        let mut temp = world.fetch_mut::<EventChannel<SpawnEvent>>();
                        temp.single_write(SpawnEvent);
                    }
                    _ => {},
                }
                Trans::None
            }
            StateEvent::Input(InputEvent::MouseButtonReleased(_)) => {
                let world = data.world;
                let transform = get_mouse_position(world);
                let (x,y) = Ground::trans_to_tile(&transform);
                let pos = (x.round() as isize,y.round() as isize);
                let mut ground = world.fetch_mut::<Ground>();
                let current = ground.map(pos).unwrap();
                *ground.map_mut(pos).unwrap() = !current;
                Trans::None
            },
            _  =>Trans::None,
        }
    }
}
