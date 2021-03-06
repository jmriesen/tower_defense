use amethyst::{
    core::{timing::Time, ArcThreadPool},
    ecs::prelude::*,
    ecs::{Entities, Join, ReadStorage},
    input::{is_close_requested, is_key_down, InputEvent, VirtualKeyCode},
    prelude::*,
    shrev::EventChannel,
};

use crate::enemy::{Enemy, SpawnEvent};
use crate::tower::BulletLaunching;
use core::time::Duration;

use crate::player::Player;

use super::utility::get_mouse_position;

mod ui;
use ui::Ui;
mod round;
use round::Round;

#[derive(Default)]
pub struct Playing<'a, 'b> {
    ui: Option<Ui>,
    dispatcher: Option<Dispatcher<'a, 'b>>,
    round: Round,
    time_since_last_round: Option<Duration>,
}

impl Playing<'_, '_> {
    fn start_next_round(&mut self, world: &mut World) {
        let mut temp = world.fetch_mut::<EventChannel<SpawnEvent>>();
        temp.single_write(self.round.advance());
        self.time_since_last_round = Some(Default::default());
    }
}

impl<'a, 'b> SimpleState for Playing<'a, 'b> {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let mut world = data.world;

        self.ui = Some(Ui::new(&mut world));

        world.insert(Player {
            money: 10,
            lives: 5,
        });

        let mut dispatcher_builder = DispatcherBuilder::new();
        dispatcher_builder.add(crate::movement::MovementSystem, "MovementSystem", &[]);
        dispatcher_builder.add(crate::tower::aiming::AimingSystem, "AimingSystem", &[]);
        dispatcher_builder.add(
            crate::movement::path::PathFollowingSystem,
            "pathFollowingSystem",
            &[],
        );
        dispatcher_builder.add(
            crate::tower::fireing_system::FireingSystem,
            "FireingSystem",
            &[],
        );
        dispatcher_builder.add(crate::collitions::CollitionSystem, "CollitionSystem", &[]);
        dispatcher_builder.add(
            crate::remove_off_screen_things::Destry,
            "cleanOfScreen",
            &[],
        );
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
        match event {
            StateEvent::Window(event) => {
                // Check if the window should be closed
                if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
                    Trans::Quit
                } else {
                    Trans::None
                }
            }
            StateEvent::Input(InputEvent::ActionReleased(action)) => match action.as_str() {
                "shoot" => {
                    let world = data.world;
                    self.start_next_round(world);
                    Trans::None
                }
                "quit" => Trans::Pop,
                _ => Trans::None,
            },
            StateEvent::Input(InputEvent::MouseButtonReleased(_)) => {
                let mut world = data.world;
                let mut player = world.fetch_mut::<Player>();
                let tower_cost = 5;
                if player.money >= tower_cost {
                    player.money -= tower_cost;
                    drop(player);
                    let transform = get_mouse_position(world);
                    BulletLaunching::create(&mut world, transform);
                } else {
                    println!(
                        "insufficient funds have {} need {}",
                        player.money, tower_cost
                    )
                }

                Trans::None
            }
            _ => Trans::None,
        }
    }

    fn update(&mut self, data: &mut StateData<'_, GameData>) -> SimpleTrans {
        if let Some(dispatcher) = self.dispatcher.as_mut() {
            dispatcher.dispatch(&data.world);
        }
        Trans::None
    }

    fn fixed_update(&mut self, data: StateData<'_, GameData>) -> SimpleTrans {
        if let Some(time_since) = self.time_since_last_round {
            if time_since > self.round.time_between_rounds {
                self.start_next_round(data.world)
            } else {
                self.time_since_last_round =
                    Some(time_since + data.world.fetch::<Time>().fixed_time());
            }
        }
        let world = data.world;
        if let Some(ui) = &self.ui {
            ui.update(world);
        }
        let player = world.read_resource::<Player>();

        if player.lives == 0 {
            Trans::Pop
        } else {
            Trans::None
        }
    }

    fn on_stop(&mut self, data: StateData<'_, GameData>) {
        let world = data.world;
        if let Some(ui) = self.ui.take() {
            ui.on_stop(world);
        }

        let (entities, enemies, towers): (
            Entities,
            ReadStorage<Enemy>,
            ReadStorage<BulletLaunching>,
        ) = world.system_data();
        for (entity, _) in (&entities, &enemies).join() {
            let _ = entities.delete(entity);
        }
        for (entity, _) in (&entities, &towers).join() {
            let _ = entities.delete(entity);
        }
    }
}
