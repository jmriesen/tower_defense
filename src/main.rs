use amethyst::{
    core::transform::TransformBundle,
    input::{InputBundle, StringBindings},
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    tiles::RenderTiles2D,
    ui::{RenderUi, UiBundle},
    utils::application_root_dir,
};
use structopt::StructOpt;

mod collitions;
mod enemy;
mod ground;
mod movement;
mod player;
mod remove_off_screen_things;
mod sprites_management;
mod state;
mod tower;

#[derive(StructOpt)]
struct Cli {
    #[structopt(parse(from_os_str))]
    leval: std::path::PathBuf,
    #[structopt(short, long)]
    edit: bool,
}

fn main() -> amethyst::Result<()> {
    let args = Cli::from_args();
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;

    let resources = app_root.join("assets");
    let display_config = app_root.join("config/display_config.ron");
    let key_bindings_path = app_root.join("config/input.ron");

    let game_data = GameDataBuilder::default()
        // For transform  position data
        .with_bundle(TransformBundle::new())?
        // For reading key bindings to events
        .with_bundle(
            InputBundle::<StringBindings>::new().with_bindings_from_file(&key_bindings_path)?,
        )?
        .with_bundle(enemy::EnemyBundle)?
        // Allows for printing text to the screen
        .with_bundle(UiBundle::<StringBindings>::new())?
        // For rendering to the screen
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config)?
                        .with_clear([0.34, 0.36, 0.52, 1.0]),
                )
                .with_plugin(RenderUi::default())
                .with_plugin(RenderFlat2D::default())
                .with_plugin(RenderTiles2D::<ground::tiles::TileRenderer>::default()),
        )?;

    // States for state machine
    let state: Box<dyn State<_, _>> = if args.edit {
        Box::new(state::Editing::default())
    } else {
        Box::new(state::Playing::default())
    };

    let mut game = Application::build(resources, state::LoadLevel::new(args.leval, state))?
        .build(game_data)?;
    game.run();

    Ok(())
}
