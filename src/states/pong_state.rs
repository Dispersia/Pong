use crate::archetypes::{ball::BallArchetype, camera::CameraArchetype, paddle::PaddleArchetype};
use crate::components::{
    ball::Ball,
    paddle::{Paddle, Side},
    velocity::Velocity
};
use amethyst::prelude::*;

pub const ARENA_HEIGHT: f32 = 100.0;
pub const ARENA_WIDTH: f32 = 100.0;

pub struct PongState;

impl SimpleState for PongState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        world.register::<Paddle>();
        world.register::<Ball>();
        world.register::<Velocity>();

        CameraArchetype::new(world);
        BallArchetype::new(world);
        PaddleArchetype::new(world, Side::Left);
        PaddleArchetype::new(world, Side::Right);
    }
}
