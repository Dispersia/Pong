use crate::states::pong_state::{ARENA_HEIGHT, ARENA_WIDTH};

use amethyst::ecs::{World, Entity};
use amethyst::prelude::*;
use amethyst::core::transform::Transform;
use amethyst::renderer::{
    Camera, Projection
};

pub struct CameraArchetype;

impl CameraArchetype {
    pub fn new(world: &mut World) -> Entity {
        let mut transform = Transform::default();

        world
            .create_entity()
            .with(Camera::from(Projection::orthographic(
                0.0,
                ARENA_WIDTH,
                0.0,
                ARENA_HEIGHT,
            )))
            .with(transform)
            .build()
    }
}