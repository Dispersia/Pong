use amethyst::ecs::{World, Entity};
use crate::components::paddle;
use amethyst::prelude::*;

pub struct PaddleArchetype;

impl PaddleArchetype {
    pub fn new(world: &mut World, side: paddle::Side) -> Entity {
        world
            .create_entity()
            .with(paddle::Paddle::new(side))
            .build()
    }
}