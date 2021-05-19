use amethyst::{
    input::{is_close_requested, is_key_down, VirtualKeyCode, InputEvent},
    prelude::*,
    shrev::{EventChannel},
    utils::application_root_dir,
};
use super::enemy;
use super::enemy::{EnemyFactory,Enemy};
use super::tower::{Tower,Bullet};
use super::ground::{Ground};
use super::sprites_management::{SpriteReasorces};



#[derive(Default)]
pub struct MyState;

impl SimpleState for MyState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        //TODO This bit is repetitive I wonder if I can write a macro for it?
        let enemy_sprite = SpriteReasorces::<Enemy>::new(world,"enemy");
        world.insert(enemy_sprite);
        let tower_sprite = SpriteReasorces::<Tower>::new(world,"tower");
        world.insert(tower_sprite);

        let bullet_sprite = SpriteReasorces::<Bullet>::new(world,"bullet");
        world.insert(bullet_sprite);
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
                        let mut temp = world.fetch_mut::<EventChannel<enemy::SpawnEvent>>();
                        temp.single_write(enemy::SpawnEvent);
                    }
                    _ => {},
                }
                Trans::None

            }
            StateEvent::Input(InputEvent::MouseButtonReleased(_)) => {
                let world = data.world;
                let mut temp = world.fetch_mut::<EventChannel<super::mouse_system::PlaceTower>>();
                temp.single_write(super::mouse_system::PlaceTower);
                Trans::None
            }
            StateEvent::Ui(_) =>{
                println!("ui UiEvent");
                Trans::None
            }
            _  =>Trans::None,
        }
    }
}

