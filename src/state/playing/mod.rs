use amethyst::{
    input::{is_close_requested, is_key_down, VirtualKeyCode, InputEvent},
    prelude::*,
    shrev::{EventChannel},
    ecs::{Entities,Join,ReadStorage},
    ecs::prelude::*,
    core::{ArcThreadPool},
    assets::{
        AssetStorage, Handle, Prefab, PrefabData, PrefabLoader, PrefabLoaderSystemDesc,
        ProgressCounter, RonFormat,
    },
};

use core::time::Duration;

use crate::enemy::{Enemy,SpawnEvent,SpawnConfig};
use crate::tower::Tower;

use crate::player::{Player};


use super::utility::{
    get_mouse_position,
};


mod ui;
use ui::UI;

#[derive(Default)]
pub struct Playing<'a,'b>{
    ui:Option<UI>,
    dispatcher: Option<Dispatcher<'a, 'b>>,
    round:usize
}

impl <'a,'b>SimpleState for Playing<'a,'b> {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let mut world = data.world;
        self.ui = Some(UI::new(&mut world));


        world.insert(Player{money:10,lives:5});

        let mut dispatcher_builder = DispatcherBuilder::new();
        dispatcher_builder.add(crate::movement::MovementSystem, "MovementSystem", &[]);
        dispatcher_builder.add(crate::tower::aiming::AimingSystem, "AimingSystem", &[]);
        dispatcher_builder.add(crate::movement::path::PathFollowingSystem, "pathFollowingSystem", &[]);
        dispatcher_builder.add(crate::tower::fireing_system::FireingSystem, "FireingSystem", &[]);
        dispatcher_builder.add(crate::collitions::CollitionSystem, "CollitionSystem", &[]);
        dispatcher_builder.add(crate::remove_off_screen_things::Destry, "cleanOfScreen", &[]);
        /*crate::enemy::EnemyBundle
            .build(&mut world, &mut dispatcher_builder)
            .expect("Failed to register PongSystemsBundle");
         */
            let mut dispatcher = dispatcher_builder
            .with_pool((*world.read_resource::<ArcThreadPool>()).clone())
            .build();
        dispatcher.setup(world);

        self.dispatcher = Some(dispatcher);
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
                        self.round+=1;
                        temp.single_write(
                            SpawnEvent{
                                number:self.round*self.round,
                                spacing :Duration::from_secs(1),
                                config:SpawnConfig{helth:3*self.round}
                            });
                        Trans::None
                    }
                    "quit" =>{
                        Trans::Pop
                    }
                    _ => {Trans::None},
                }

            }
            StateEvent::Input(InputEvent::MouseButtonReleased(_)) => {
                let mut world = data.world;
                let mut player = world.fetch_mut::<Player>();
                let tower_cost = 5;
                if player.money>= tower_cost{
                    player.money-= tower_cost;
                    drop(player);
                    let transform = get_mouse_position(world);
                    Tower::create(&mut world, transform);
                }else{
                    println!("insufficient funds have {} need {}",player.money, tower_cost)
                }

                Trans::None
            },
            _  =>Trans::None,
        }
    }

    fn update(&mut self, data: &mut StateData<'_, GameData>) -> SimpleTrans{
        if let Some(dispatcher) = self.dispatcher.as_mut() {
            dispatcher.dispatch(&data.world);
        }
        Trans::None
    }
    fn fixed_update(&mut self, data: StateData<'_, GameData>) -> SimpleTrans{
        let world = data.world;
        if let Some(ui) = &self.ui{
            ui.update(world);
        }
        let player = world.read_resource::<Player>();

        if player.lives == 0{
            Trans::Pop
        }else{
            Trans::None
        }
    }

    fn on_stop(&mut self, data: StateData<'_, GameData>){
        let world = data.world;
        if let Some(ui) = self.ui.take(){
            ui.on_stop(world);
        }

        let (entities,enemies,towers):
        (
            Entities,
            ReadStorage<Enemy>,
            ReadStorage<Tower>,
        ) = world.system_data();
        for (entity, _) in  (&entities, &enemies).join(){
            let _ = entities.delete(entity);
        }
        for (entity, _) in (&entities, &towers).join(){
            let _ = entities.delete(entity);
        }
   }
}
