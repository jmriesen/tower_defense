use amethyst::{
    input::{is_close_requested, is_key_down, VirtualKeyCode, InputEvent},
    prelude::*,
    shrev::{EventChannel},
    utils::application_root_dir,
};

use crate::enemy::{EnemyFactory,SpawnEvent};
use crate::tower::Tower;
use crate::ground::{Ground};
use crate::player::{Money, set_up_money};


use super::utility::{
    get_mouse_position,
    set_up_sprites,
};




#[derive(Default)]
pub struct Playing;

impl SimpleState for Playing {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        set_up_sprites(world);

        set_up_money(world);
        //TODO This bit is repetitive I wonder if I can write a macro for it?
                //TODO all the rest of this should be dealt with by a config file.
        /*
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
        */
        let mut ground = Ground::load(application_root_dir().unwrap().join("ground.ron")).unwrap();
        ground.refresh();
        ground.create_tile_map(world);
        ground.create_camera(world);

        world.insert(ground);

        world.create_entity()
            .with(Ground::tile_to_trans((4,5)))
            .with(EnemyFactory)
            .build();
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
                let mut world = data.world;
                let mut money = world.fetch_mut::<Money>();
                let tower_cost = 5;
                if money.amount >= tower_cost{
                    money.amount -= tower_cost;
                    drop(money);
                    let transform = get_mouse_position(world);
                    Tower::create(&mut world, transform);
                }else{
                    println!("insufficient funds have {} need {}",money.amount, tower_cost)
                }

                Trans::None
            },
            _  =>Trans::None,
        }
    }
}
