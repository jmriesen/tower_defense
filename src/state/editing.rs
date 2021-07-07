use amethyst::{
    input::{is_close_requested, is_key_down, VirtualKeyCode, InputEvent},
    prelude::*,
};


use crate::ground::{
    Ground,
    Tile,
};



use super::utility::{
    get_mouse_position,
};

#[derive(Default)]
pub struct Editing;

impl SimpleState for Editing{
    fn on_start(&mut self, _data: StateData<'_, GameData<'_, '_>>) {
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
                    "play" =>{
                        Trans::Push(Box::new(super::Playing::default()))
                    }
                    "quit" =>{
                        Trans::Pop
                    }
                    "save" =>{
                        let world = data.world;
                        let ground = world.fetch::<Ground>();
                        ground.save();
                        Trans::None
                    }
                    _ => {Trans::None},
                }
            }
            StateEvent::Input(InputEvent::MouseButtonReleased(_)) => {
                let world = data.world;
                let transform = get_mouse_position(world);
                let mut ground = world.fetch_mut::<Ground>();
                let pos = transform.into();
                let current = ground.map(pos).unwrap();
                *ground.map_mut(pos).unwrap() =
                    match current{
                        Tile::Grass => Tile::Path,
                        Tile::Path => Tile::Water,
                        Tile::Water => Tile::Grass,
                    };
                Trans::None
            },
            _  =>Trans::None,
        }
    }
}
