//#![windows_subsystem = "windows"]

mod archetypes;
mod components;
mod resources;
mod states;
mod systems;

use crate::systems::{
    BounceSystem, BoundConstraintSystem, MovementSystem, PongInputSystem, ScoreSystem,
};
use amethyst::{
    core::{
        bundle::SystemBundle, frame_limiter::FrameRateLimitStrategy, transform::TransformBundle,
    },
    ecs::prelude::DispatcherBuilder,
    error::Error,
    input::InputBundle,
    prelude::*,
    renderer::{
        DisplayConfig, DrawDebugLines, DrawFlat2D, Pipeline, PosColorNorm, RenderBundle, Stage,
    },
    utils::application_root_dir,
};

use std::time::Duration;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let root = application_root_dir()?;

    let config_path = root.join("resources").join("display_config.ron");

    let config = DisplayConfig::load(&config_path);

    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target([0.0, 0.0, 0.0, 1.0], 1.0)
            .with_pass(DrawFlat2D::new())
            .with_pass(DrawDebugLines::<PosColorNorm>::new()),
    );

    let binding_path = root.join("resources").join("bindings_config.ron");

    let input_bundle =
        InputBundle::<String, String>::new().with_bindings_from_file(binding_path)?;

    let game_data = GameDataBuilder::default()
        .with_bundle(input_bundle)?
        .with_bundle(PongBundle)?
        .with_bundle(RenderBundle::new(pipe, Some(config)).with_sprite_sheet_processor())?
        .with_bundle(TransformBundle::new().with_dep(&["movement_system"]))?;

    let mut game = Application::build(root, states::pong_state::PongState)?
        .with_frame_limit(
            FrameRateLimitStrategy::SleepAndYield(Duration::from_millis(2)),
            144,
        )
        .build(game_data)?;

    game.run();

    Ok(())
}

struct PongBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for PongBundle {
    fn build(self, builder: &mut DispatcherBuilder<'a, 'b>) -> Result<(), Error> {
        builder.add(PongInputSystem, "pong_input_system", &[]);
        builder.add(MovementSystem, "movement_system", &["pong_input_system"]);
        builder.add(
            BoundConstraintSystem,
            "bound_constraint_system",
            &["movement_system"],
        );
        builder.add(BounceSystem, "bounce_system", &["movement_system"]);
        builder.add(ScoreSystem, "score_system", &["movement_system"]);

        #[cfg(feature = "draw_debug")]
        use crate::systems::PongDebugLinesSystem;
        #[cfg(feature = "draw_debug")]
        builder.add(
            PongDebugLinesSystem,
            "debug_lines_system",
            &["movement_system"],
        );

        Ok(())
    }
}
