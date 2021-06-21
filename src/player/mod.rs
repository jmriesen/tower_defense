use amethyst::{
    ecs::{Read,WriteStorage,Entity,System},
    assets::{Loader},
    ui::{Anchor, FontHandle, LineMode, TtfFormat, UiText, UiTransform},
};
use amethyst::prelude::{Builder, World, WorldExt};

#[derive(Default)]
pub struct Money{
    pub amount:usize,
    lable: Option<Entity>,
}
pub fn set_up_money(world:&mut World){
    let ui_transform = UiTransform::new(
        String::from("simple_button"), // id
        Anchor::TopLeft,                // anchor
        Anchor::TopLeft,                // pivot
        0f32,                          // x
        0f32,                          // y
        0f32,                          // z
        200f32,                        // width
        30f32,                         // height
    );
    let font: FontHandle = world.read_resource::<Loader>().load(
        "fonts/Bangers-Regular.ttf",
        TtfFormat,
        (),
        &world.read_resource(),
    );
    let ui_text = UiText::new(
        font,                   // font
        String::from("Simple Button"), // text
        [1.0, 1.0, 1.0, 0.5],          // color
        31f32,                         // font_size
        LineMode::Single,              // line mode
        Anchor::Middle,                // alignment
    );
    let lable = Some(world.create_entity()
        .with(ui_transform)
        .with(ui_text)
        .build());

    world.insert(Money{amount:10,lable});
}
pub struct MoneyDesplay;

impl<'s> System<'s> for MoneyDesplay{
    type SystemData = (
        Read<'s, Money>,
        WriteStorage<'s, UiText>,
    );

    fn run(&mut self, (money, mut text): Self::SystemData) {
        if let Some(lable) = money.lable{
            let lable = text.get_mut(lable).unwrap();
            lable.text = format!("Money ${}",money.amount);
        }
    }
}

