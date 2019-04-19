mod archetypes;
mod components;
mod resources;
mod states;
mod systems;

use crate::systems::{MovementSystem, PaddleSystem};
use amethyst::{
    core::{
        bundle::SystemBundle, frame_limiter::FrameRateLimitStrategy, transform::TransformBundle,
    },
    ecs::prelude::DispatcherBuilder,
    error::Error,
    prelude::*,
    renderer::{DisplayConfig, DrawFlat, Pipeline, PosNormTex, RenderBundle, Stage},
    utils::application_root_dir
};


use std::time::Duration;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let root = application_root_dir()?;

    let path = root.join("resources/display_config.ron");

    let config = DisplayConfig::load(&path);

    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target([0.0, 0.0, 0.0, 1.0], 1.0)
            .with_pass(DrawFlat::<PosNormTex>::new()),
    );

    let game_data = GameDataBuilder::default()
        .with_bundle(PongBundle)?
        .with_bundle(RenderBundle::new(pipe, Some(config)).with_sprite_sheet_processor())?
        .with_bundle(TransformBundle::new())?;

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
        builder.add(PaddleSystem, "paddle_system", &[]);
        builder.add(MovementSystem, "movement_system", &["paddle_system"]);
        Ok(())
    }
}