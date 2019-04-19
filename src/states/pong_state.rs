use crate::archetypes::{ball::BallArchetype, camera::CameraArchetype, paddle::PaddleArchetype};
use crate::components::{
    ball::Ball,
    paddle::{Paddle, Side},
    velocity::Velocity,
};
use crate::resources::spritesheet;
use amethyst::prelude::*;

pub const ARENA_HEIGHT: f32 = 100.0;
pub const ARENA_WIDTH: f32 = 100.0;

pub struct PongState;

impl SimpleState for PongState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        let sprite_sheet_handle = spritesheet::load_sprite_sheet(world);

        world.register::<Paddle>();
        world.register::<Ball>();
        world.register::<Velocity>();

        BallArchetype::new(world);

        PaddleArchetype::new(world, &sprite_sheet_handle, Side::Left);
        PaddleArchetype::new(world, &sprite_sheet_handle, Side::Right);

        CameraArchetype::new(world);
    }
}
