use tower_def::TowerDefState;

use amethyst::{
    core::transform::TransformBundle,
    input::{InputBundle, StringBindings},
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    utils::application_root_dir,
};

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;

    let assets_dir = app_root.join("assets");
    let config_dir = app_root.join("config");
    let display_config_path = config_dir.join("display.ron");

    let input_bundle = InputBundle::<StringBindings>::new();

    let game_data = GameDataBuilder::default()
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)?
                        .with_clear([0.34, 0.36, 0.52, 1.0]),
                )
                .with_plugin(RenderFlat2D::default()),
        )?
        .with_bundle(input_bundle)?
        .with(tower_def::runner::RunnerSystem, "runner_sytem", &[])
        .with(tower_def::runner::SpawnSystem::new(), "spawn_system", &[])
        .with(tower_def::tower::TowerSystem, "tower_sytem", &[])
        .with(tower_def::tower::MissleSystem, "missle_sytem", &[])
        .with(
            tower_def::tower::BuildPointSystem,
            "build_point_system",
            &["input_system"],
        )
        .with_bundle(TransformBundle::new())?;

    let mut game = Application::new(assets_dir, TowerDefState {}, game_data)?;
    game.run();

    Ok(())
}
