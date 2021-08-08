use amethyst::{
    assets::Loader,
    ecs::{Entities, Entity, Read, WriteStorage},
    prelude::*,
    ui::{Anchor, FontHandle, LineMode, TtfFormat, UiText, UiTransform},
};
pub struct Ui {
    money: Entity,
    lives: Entity,
}
use crate::player::Player;
impl Ui {
    pub fn new(world: &mut World) -> Self {
        let font: FontHandle = world.read_resource::<Loader>().load(
            "fonts/Bangers-Regular.ttf",
            TtfFormat,
            (),
            &world.read_resource(),
        );

        // components
        let money_transform = UiTransform::new(
            String::from("Money Label"), // id
            Anchor::TopLeft,             // anchor
            Anchor::TopLeft,             // pivot
            0f32,                        // x
            0f32,                        // y
            0f32,                        // z
            200f32,                      // width
            30f32,                       // height
        );
        let money_text = UiText::new(
            font.clone(),          // font
            String::from("Money"), // text
            [1.0, 1.0, 1.0, 0.5],  // color
            31f32,                 // font_size
            LineMode::Single,      // line mode
            Anchor::Middle,        // alignment
        );

        // Entity
        let money = world
            .create_entity()
            .with(money_transform)
            .with(money_text)
            .build();

        let lives_transform = UiTransform::new(
            String::from("Money Label"), // id
            Anchor::TopLeft,             // anchor
            Anchor::TopLeft,             // pivot
            0f32,                        // x
            -30f32,                      // y
            0f32,                        // z
            200f32,                      // width
            30f32,                       // height
        );
        let lives_text = UiText::new(
            font,                  // font
            String::from("Money"), // text
            [1.0, 1.0, 1.0, 0.5],  // color
            31f32,                 // font_size
            LineMode::Single,      // line mode
            Anchor::Middle,        // alignment
        );
        let lives = world
            .create_entity()
            .with(lives_transform)
            .with(lives_text)
            .build();
        Ui { money, lives }
    }

    pub fn update(&self, world: &mut World) {
        let (player, mut text): (Read<Player>, WriteStorage<UiText>) = world.system_data();

        let lable = text.get_mut(self.money).unwrap();
        lable.text = format!("Money ${}", player.money);

        let lable = text.get_mut(self.lives).unwrap();
        lable.text = format!("Lives {}", player.lives);
    }

    pub fn on_stop(self, world: &mut World) {
        let entities: Entities = world.system_data();
        let _ = entities.delete(self.money);
        let _ = entities.delete(self.lives);
    }
}
