use amethyst::{
    core::transform::TransformBundle,
    input::{InputBundle, StringBindings},
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    tiles::{RenderTiles2D},
    ui::{RenderUi, UiBundle},
    utils::application_root_dir,
};

mod state;
mod enemy;
mod tower;
mod movement;
mod ground;
mod collitions;
mod sprites_management;
mod player;
mod remove_off_screen_things;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;

    let resources = app_root.join("assets");
    let display_config = app_root.join("config/display_config.ron");
    let key_bindings_path = app_root.join("config/input.ron");

    let game_data = GameDataBuilder::default()
        .with_bundle(TransformBundle::new())?
        .with_bundle(
            InputBundle::<StringBindings>::new().with_bindings_from_file(&key_bindings_path)?,
        )?
        .with(movement::path::PathFollowingSystem, "pathFollowingSystem", &[])
        .with(tower::fireing_system::FireingSystem, "FireingSystem", &[])
        .with(movement::MovementSystem, "MovementSystem", &[])
        .with(collitions::CollitionSystem, "CollitionSystem", &[])
        .with(tower::aiming::AimingSystem, "AimingSystem", &[])
        .with(enemy::DeathSystem, "DeathSystem", &[])
        .with(player::MoneyDesplay, "DesplaySystem", &[])
        .with(remove_off_screen_things::Destry, "cleanOfScreen", &[])
        .with_bundle(enemy::MyBundle)?
        //.with_bundle(mouse_system::MyBundle)?
        .with_bundle(UiBundle::<StringBindings>::new())?
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config)?
                        .with_clear([0.34, 0.36, 0.52, 1.0]),
                )
                .with_plugin(RenderUi::default())
                .with_plugin(RenderFlat2D::default())
                .with_plugin(RenderTiles2D::<ground::tiles::GroundTile>::default())

        )?;

    let mut game = Application::new(resources, state::MyState::default(), game_data)?;
    game.run();

    Ok(())
}
