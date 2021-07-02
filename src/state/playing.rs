use amethyst::{
    input::{is_close_requested, is_key_down, VirtualKeyCode, InputEvent},
    prelude::*,
    shrev::{EventChannel},
    ui::{Anchor, FontHandle, LineMode, TtfFormat, UiText, UiTransform},
    ecs::{Entities,Read,WriteStorage,Entity,Join,ReadStorage},

    assets::{Loader},
};



use crate::enemy::{Enemy,SpawnEvent};
use crate::tower::Tower;

use crate::player::{Player};


use super::utility::{
    get_mouse_position,
};




#[derive(Default)]
pub struct Playing{
    money: Option<Entity>,
    lives: Option<Entity>,
}

impl SimpleState for Playing {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        let font: FontHandle = world.read_resource::<Loader>().load(
            "fonts/Bangers-Regular.ttf",
            TtfFormat,
            (),
            &world.read_resource(),
        );

    let money_transform = UiTransform::new(
        String::from("Money Label"), // id
        Anchor::TopLeft,                // anchor
        Anchor::TopLeft,                // pivot
        0f32,                          // x
        0f32,                          // y
        0f32,                          // z
        200f32,                        // width
        30f32,                         // height
    );
   let money_text = UiText::new(
       font.clone(),                   // font
        String::from("Money"), // text
        [1.0, 1.0, 1.0, 0.5],          // color
        31f32,                         // font_size
        LineMode::Single,              // line mode
        Anchor::Middle,                // alignment
    );
        self.money = Some(world.create_entity()
                          .with(money_transform)
                          .with(money_text)
                          .build());
        let lives_transform = UiTransform::new(
            String::from("Money Label"), // id
            Anchor::TopLeft,                // anchor
            Anchor::TopLeft,                // pivot
            0f32,                          // x
            -30f32,                          // y
            0f32,                          // z
            200f32,                        // width
            30f32,                         // height
        );
        let lives_text = UiText::new(
            font,                   // font
            String::from("Money"), // text
            [1.0, 1.0, 1.0, 0.5],          // color
            31f32,                         // font_size
            LineMode::Single,              // line mode
            Anchor::Middle,                // alignment
        );
        self.lives= Some(world.create_entity()
                          .with(lives_transform)
                          .with(lives_text)
                          .build());

        world.insert(Player{money:10,lives:5});

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
    fn fixed_update(&mut self, data: StateData<'_, GameData>) -> SimpleTrans{
        let world = data.world;
        let (player,mut text):(
            Read<Player>,
            WriteStorage<UiText>
        ) = world.system_data();
        if let Some(lable) = self.money{
            let lable = text.get_mut(lable).unwrap();
            lable.text = format!("Money ${}",player.money);
        }

        if let Some(lable) = self.lives{
            let lable = text.get_mut(lable).unwrap();
            lable.text = format!("Lives {}",player.lives);
        }
        if player.lives == 0{
            Trans::Pop
        }else{
            Trans::None
        }
    }

    fn on_stop(&mut self, data: StateData<'_, GameData>){
        let world = data.world;
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
        if let Some(entity) = self.money{
            let _ = entities.delete(entity);
        }
        if let Some(entity) = self.lives{
            let _ = entities.delete(entity);
        }

    }
}
