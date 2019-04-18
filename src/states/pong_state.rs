use amethyst::{
    prelude::*
};
use crate::components::{
    paddle::Side
};
use crate::archetypes::{
    camera::CameraArchetype,
    ball::BallArchetype,
    paddle::PaddleArchetype,
};

pub struct PongState;

impl SimpleState for PongState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        CameraArchetype::new(world);
        BallArchetype::new(world);
        PaddleArchetype::new(world, Side::Left);
        PaddleArchetype::new(world, Side::Right);
    }
}