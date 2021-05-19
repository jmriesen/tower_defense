use amethyst::{
    assets::{AssetStorage, Loader,Handle},
    prelude::*,
    renderer::{ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
};

use std::marker::PhantomData;

pub struct SpriteReasorces<T> {
    sprites:Vec<SpriteRender>,
    phantom: PhantomData<T>,
}

impl <T>SpriteReasorces<T> {
    pub fn new(world: &mut World,sprite:&str)->Self{
        Self{
            sprites:load_sprites(world,sprite),
            phantom:PhantomData
        }
    }
    pub fn get(&self,index:usize)->SpriteRender{
        self.sprites[index].clone()
    }
}



pub  fn load_sheet(world: &mut World,file_name:&str) -> Handle<SpriteSheet> {
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
fn load_sprites(world: &mut World,sprite:&str) -> Vec<SpriteRender> {
    let sheet_handle = load_sheet(world, sprite);
    (0..1)
        .map(|i| SpriteRender {
            sprite_sheet: sheet_handle.clone(),
            sprite_number: i,
        })
        .collect()
}