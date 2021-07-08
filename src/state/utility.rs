use amethyst::{prelude::*,
    ecs::{
        ReadStorage,
        Join,
        Read,
        ReadExpect,
        WriteStorage,
    },
    core::{
        transform::Transform,
        math::{Point3},
    },
    renderer::{Camera},
    input::{InputHandler,StringBindings},
    window::ScreenDimensions,
};

pub fn get_mouse_position(world:&World)->Transform{
    let (camras,transfroms,input,dimensions):
    (
        ReadStorage<Camera>,
        WriteStorage<Transform>,
        Read<InputHandler<StringBindings>>,
        ReadExpect<ScreenDimensions>,
    ) = world.system_data();
    let point = {
        //Only supporting one camra at the moment.
        let (camra, transform) = (&camras, &transfroms).join().next().unwrap();
        let (x,y) = input.mouse_position().unwrap();
        camra.screen_to_world_point(
            Point3::new(x, y, 1.0),
            dimensions.diagonal(),
            transform
        )
    };
    let mut transform = Transform::default();
    transform.set_translation_xyz(point.x ,point.y,0.);
    transform
}
pub fn set_up_sprites(world:&mut World){
    use crate::sprites_management::{SpriteReasorces};
    use crate::enemy::Enemy;
    use crate::tower::{Tower,Bullet};

    let enemy_sprite = SpriteReasorces::<Enemy>::new(world,"enemy");
    world.insert(enemy_sprite);
    let tower_sprite = SpriteReasorces::<Tower>::new(world,"tower");
    world.insert(tower_sprite);

    let bullet_sprite = SpriteReasorces::<Bullet>::new(world,"bullet");
    world.insert(bullet_sprite);
}
