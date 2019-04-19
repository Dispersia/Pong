use crate::components::paddle;
use crate::states::pong_state::{ARENA_HEIGHT, ARENA_WIDTH};
use amethyst::core::transform::Transform;
use amethyst::ecs::{Entity, World};
use amethyst::prelude::*;

pub struct PaddleArchetype;

impl PaddleArchetype {
    pub fn new(world: &mut World, side: paddle::Side) -> Entity {
        let mut transform = Transform::default();

        let x: f32;

        if side == paddle::Side::Left {
            x = 5.;
        } else {
            x = 4.;
        }

        transform.set_translation_xyz(x, ARENA_HEIGHT / 2., 0.);

        world
            .create_entity()
            .with(paddle::Paddle::new(side))
            .with(transform)
            .build()
    }
}
