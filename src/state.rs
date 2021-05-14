use amethyst::{
    assets::{AssetStorage, Loader,Handle},
    core::transform::Transform,
    input::{is_close_requested, is_key_down, VirtualKeyCode, InputEvent},
    prelude::*,
    renderer::{ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
    window::ScreenDimensions,
    shrev::{EventChannel},
};

use std::time::Duration;
use super::enemy;
use super::enemy::EnemyFactory;
use super::tower::Tower;
use super::ground::{Ground};

/// A dummy game state that shows 3 sprites.
#[derive(Default)]
pub struct MyState;

impl SimpleState for MyState {
    // Here, we define hooks that will be called throughout the lifecycle of our game state.
    //
    // In this example, `on_start` is used for initializing entities
    // and `handle_state` for managing the state transitions.
    //
    // For more state lifecycle hooks, see:
    // https://book.amethyst.rs/stable/concepts/state.html#life-cycle

    /// The state is initialized with:
    /// - a camera centered in the middle of the screen.
    /// - 3 sprites places around the center.
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        // Get the screen dimensions so we can initialize the camera and
        // place our sprites correctly later. We'll clone this since we'll
        // pass the world mutably to the following functions.
        let dimensions = (*world.read_resource::<ScreenDimensions>()).clone();

        let mut ground = Ground::new(&dimensions,10,10);
        for i in 0..9{
            ground.map_mut()[i][1] = false;
            ground.map_mut()[9-i][3] = false;
        }

        ground.sink_points_mut().push((0,0));
        ground.refresh();
        ground.create_tile_map(world);
        ground.create_camera(world);

        world.insert(Some(ground));


        // Load our sprites and display them
        let sprites = load_sprites(world);

        let mut transform = Transform::default();
        transform.set_translation_xyz(32.*4.,32.*5.,0.);
        world.create_entity()
            .with(transform )
            .with(EnemyFactory::new(sprites.clone()))
            .build();

        let transform = Transform::default();
//        transform.set_translation_xyz(400.,600.,0.);
        world.create_entity()
            .with(transform)
            .with(Tower::new(sprites,Duration::new(1, 0)))
            .build();

    }

    /// The following events are handled:
    /// - The game state is quit when either the close button is clicked or when the escape key is pressed.
    /// - Any other keypress is simply logged to the console.
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
            _ => Trans::None,
        }
    }
}

/// Creates a camera entity in the `world`.
///
/// The `dimensions` are used to center the camera in the middle
/// of the screen, as well as make it cover the entire screen.
//TODO I should mess with this and see if I can make it work more how I want.


/// Loads and splits the `logo.png` image asset into 3 sprites,
/// which will then be assigned to entities for rendering them.
///
/// The provided `world` is used to retrieve the resource loader.
pub fn load_sheet(world: &mut World,file_name:&str) -> Handle<SpriteSheet> {
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            format!("sprites/{}.png",file_name),
            ImageFormat::default(),
            (),
            &texture_storage,
        )
    };

    let loader = world.read_resource::<Loader>();
    let sheet_storage = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        format!("sprites/{}.ron",file_name),
        SpriteSheetFormat(texture_handle),
        (),
        &sheet_storage,
    )

}
fn load_sprites(world: &mut World) -> Vec<SpriteRender> {
    let sheet_handle = load_sheet(world,"enemy");
    (0..1)
        .map(|i| SpriteRender {
            sprite_sheet: sheet_handle.clone(),
            sprite_number: i,
        })
        .collect()
}
