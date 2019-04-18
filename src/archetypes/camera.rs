use amethyst::ecs::{World, Entity};
use amethyst::prelude::*;
use amethyst::core::transform::Transform;
use amethyst::renderer::{
    Camera, Projection
};

pub const ARENA_HEIGHT: f32 = 100.0;
pub const ARENA_WIDTH: f32 = 100.0;

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