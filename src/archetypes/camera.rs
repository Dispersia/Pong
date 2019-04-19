use crate::states::pong_state::{ARENA_HEIGHT, ARENA_WIDTH};


use amethyst::core::transform::Transform;
use amethyst::ecs::{Entity, World};
use amethyst::prelude::*;
use amethyst::renderer::{Camera, Projection};

pub struct CameraArchetype;

impl CameraArchetype {
    pub fn new(world: &mut World) -> Entity {
        let mut transform = Transform::default();
        transform.set_translation_z(1.0);

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